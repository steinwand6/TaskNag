import React from 'react';
import { Task, TaskStatus, TaskNotificationSettings, Tag } from '../types/Task';
import { BrowserAction, BrowserActionSettings } from '../types/BrowserAction';
import { NotificationSettings } from './NotificationSettings';
import { TagDisplay } from './TagDisplay';
import URLActionConfig from './URLActionConfig';
import { useTaskStore } from '../stores/taskStore';
import { DEFAULT_TASK_STATUS, STATUS_OPTIONS } from '../constants/taskStatus';
import { LogService } from '../services/logService';

interface EditTaskModalProps {
  isOpen: boolean;
  onClose: () => void;
  task: Task | null;
}

export const EditTaskModal: React.FC<EditTaskModalProps> = ({ isOpen, onClose, task }) => {
  // デバッグ用: onCloseが呼ばれた時のログ
  const handleClose = () => {
    LogService.info('EditTaskModal: onCloseが呼ばれました');
    onClose();
  };
  const { updateTask, tags, loadTags, createTag } = useTaskStore();
  
  // タグ管理用状態
  const [selectedTags, setSelectedTags] = React.useState<Tag[]>([]);
  const [showTagSelector, setShowTagSelector] = React.useState(false);
  const [newTagName, setNewTagName] = React.useState('');
  const [newTagColor, setNewTagColor] = React.useState('#3b82f6');
  
  // ブラウザアクション管理用状態
  const [browserActions, setBrowserActions] = React.useState<BrowserAction[]>([]);
  const [formData, setFormData] = React.useState({
    title: '',
    description: '',
    status: DEFAULT_TASK_STATUS,
    dueDate: '',
    notificationSettings: {
      notificationType: 'none',
      level: 1,
    } as TaskNotificationSettings,
    browserActions: {
      enabled: false,
      actions: [],
    } as BrowserActionSettings,
  });
  
  // Initialize form data when task changes
  React.useEffect(() => {
    if (task) {
      console.log('EditTaskModal: Initializing with task:', task);
      console.log('EditTaskModal: task.browserActions:', task.browserActions);
      console.log('EditTaskModal: typeof task.browserActions:', typeof task.browserActions);
      
      let browserActionsData;
      if (typeof task.browserActions === 'string') {
        // JSONとしてパースする必要がある
        try {
          browserActionsData = JSON.parse(task.browserActions);
          console.log('EditTaskModal: Parsed browserActions from JSON string:', browserActionsData);
        } catch (e) {
          console.error('EditTaskModal: Failed to parse browserActions JSON:', e);
          browserActionsData = { enabled: false, actions: [] };
        }
      } else {
        browserActionsData = task.browserActions || { enabled: false, actions: [] };
      }
      console.log('EditTaskModal: browserActionsData:', browserActionsData);
      console.log('EditTaskModal: browserActionsData.actions:', browserActionsData.actions);
      
      setFormData({
        title: task.title,
        description: task.description || '',
        status: task.status,
        dueDate: task.dueDate ? task.dueDate.toISOString().split('T')[0] : '',
        notificationSettings: task.notificationSettings || {
          notificationType: 'none',
          level: 1,
        },
        browserActions: browserActionsData,
      });
      setSelectedTags(task.tags || []);
      setBrowserActions(browserActionsData.actions);
      
      console.log('EditTaskModal: Set browserActions state to:', browserActionsData.actions);
    }
  }, [task]);
  
  // Load tags when component mounts
  React.useEffect(() => {
    loadTags();
  }, [loadTags]);
  
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    LogService.info('EditTaskModal: handleSubmitが呼ばれました');
    if (!formData.title.trim() || !task) return;
    
    // 一時的なIDを持つタグを除外（実際のDBに存在するタグのみを保存）
    const validTags = selectedTags.filter(tag => !tag.id.startsWith('temp-'));
    
    console.log('EditTaskModal: Saving task with data:');
    console.log('  - formData.browserActions:', formData.browserActions);
    console.log('  - browserActions state:', browserActions);
    
    const browserActionsToSave = {
      enabled: formData.browserActions.enabled,
      actions: formData.browserActions.actions,
    };
    console.log('  - browserActionsToSave:', browserActionsToSave);
    
    updateTask(task.id, {
      title: formData.title,
      description: formData.description || undefined,
      status: formData.status,
      dueDate: formData.dueDate ? new Date(formData.dueDate) : undefined,
      notificationSettings: formData.notificationSettings,
      browserActions: browserActionsToSave,
      tags: validTags,
    });
    
    handleClose();
  };

  const handleNotificationChange = (settings: TaskNotificationSettings) => {
    LogService.info('EditTaskModal: 通知設定が変更されました', JSON.stringify(settings));
    setFormData({ ...formData, notificationSettings: settings });
  };
  
  const handleBrowserActionsChange = (actions: BrowserAction[]) => {
    setBrowserActions(actions);
    // formData.browserActions.actionsも同期更新
    setFormData(prev => ({
      ...prev,
      browserActions: {
        ...prev.browserActions,
        actions: actions,
        // ブラウザアクションが設定されている場合は自動的に有効にする
        enabled: actions.length > 0 ? true : prev.browserActions.enabled
      }
    }));
  };
  
  const handleBrowserActionToggle = (enabled: boolean) => {
    setFormData({ 
      ...formData, 
      browserActions: { ...formData.browserActions, enabled }
    });
  };
  
  // タグ関連のハンドラー
  const handleAddTag = (tag: Tag) => {
    if (!selectedTags.find(t => t.id === tag.id)) {
      setSelectedTags([...selectedTags, tag]);
    }
  };
  
  const handleRemoveTag = (tagId: string) => {
    setSelectedTags(selectedTags.filter(tag => tag.id !== tagId));
  };
  
  const handleCreateNewTag = async () => {
    if (!newTagName.trim()) return;
    
    try {
      // 実際にタグを作成してからタスクに追加
      const newTag = await createTag({
        name: newTagName.trim(),
        color: newTagColor
      });
      
      setSelectedTags([...selectedTags, newTag]);
      setNewTagName('');
      setNewTagColor('#3b82f6');
      setShowTagSelector(false);
    } catch (error) {
      console.error('Failed to create new tag:', error);
      // エラー処理：ユーザーに通知
      alert('タグの作成に失敗しました。もう一度お試しください。');
    }
  };
  
  const availableTags = tags.filter(tag => 
    !selectedTags.find(selected => selected.id === tag.id)
  );
  
  if (!isOpen || !task) return null;
  
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg w-full max-w-md max-h-[90vh] flex flex-col overflow-hidden">
        <div className="p-6 overflow-y-auto">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-lg font-semibold">タスクを編集</h2>
          <button
            onClick={handleClose}
            className="text-gray-400 hover:text-gray-600"
          >
            ×
          </button>
        </div>
        
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              タスク名 *
            </label>
            <input
              type="text"
              value={formData.title}
              onChange={(e) => setFormData({ ...formData, title: e.target.value })}
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="タスクのタイトルを入力..."
              required
            />
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              説明
            </label>
            <textarea
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              rows={3}
              placeholder="タスクの詳細説明（任意）..."
            />
          </div>
          
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                ステータス
              </label>
              <select
                value={formData.status}
                onChange={(e) => setFormData({ ...formData, status: e.target.value as TaskStatus })}
                className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                {STATUS_OPTIONS.map(({ value, label }) => (
                  <option key={value} value={value}>
                    {label}
                  </option>
                ))}
              </select>
            </div>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              期限
            </label>
            <input
              type="date"
              value={formData.dueDate}
              onChange={(e) => setFormData({ ...formData, dueDate: e.target.value })}
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          
          {/* タグ管理セクション */}
          <div className="border-t pt-4">
            <div className="flex items-center justify-between mb-2">
              <label className="block text-sm font-medium text-gray-700">
                タグ
              </label>
              <button
                type="button"
                onClick={() => setShowTagSelector(!showTagSelector)}
                className="text-sm text-blue-600 hover:text-blue-800 flex items-center gap-1"
              >
                <span>+</span> タグを追加
              </button>
            </div>
            
            {/* 現在のタグ */}
            {selectedTags.length > 0 && (
              <div className="mb-2">
                <TagDisplay 
                  tags={selectedTags}
                  maxDisplay={10}
                  size="md"
                  showRemoveButton={true}
                  onRemove={handleRemoveTag}
                />
              </div>
            )}
            
            {/* タグセレクター */}
            {showTagSelector && (
              <div className="bg-gray-50 p-3 rounded-md space-y-3">
                {/* 既存タグから選択 */}
                {availableTags.length > 0 && (
                  <div>
                    <h4 className="text-xs font-medium text-gray-600 mb-2">既存のタグから選択</h4>
                    <div className="flex flex-wrap gap-1">
                      {availableTags.map((tag) => (
                        <button
                          key={tag.id}
                          type="button"
                          onClick={() => handleAddTag(tag)}
                          className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border border-gray-200 hover:border-gray-300 hover:shadow-sm transition-all"
                          style={{
                            backgroundColor: tag.color + '10',
                            color: tag.color,
                          }}
                        >
                          {tag.name}
                        </button>
                      ))}
                    </div>
                  </div>
                )}
                
                {/* 新しいタグを作成 */}
                <div>
                  <h4 className="text-xs font-medium text-gray-600 mb-2">新しいタグを作成</h4>
                  <div className="flex gap-2">
                    <input
                      type="text"
                      placeholder="タグ名"
                      value={newTagName}
                      onChange={(e) => setNewTagName(e.target.value)}
                      className="flex-1 text-sm border border-gray-300 rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-blue-500"
                    />
                    <input
                      type="color"
                      value={newTagColor}
                      onChange={(e) => setNewTagColor(e.target.value)}
                      className="w-8 h-7 border border-gray-300 rounded cursor-pointer"
                    />
                    <button
                      type="button"
                      onClick={handleCreateNewTag}
                      disabled={!newTagName.trim()}
                      className="px-2 py-1 text-xs bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      追加
                    </button>
                  </div>
                </div>
                
                <button
                  type="button"
                  onClick={() => setShowTagSelector(false)}
                  className="text-xs text-gray-500 hover:text-gray-700"
                >
                  閉じる
                </button>
              </div>
            )}
          </div>
          
          {/* 通知設定セクション */}
          <div className="border-t pt-4">
            <NotificationSettings
              settings={formData.notificationSettings}
              onChange={handleNotificationChange}
              hasDueDate={!!formData.dueDate}
            />
          </div>
          
          {/* ブラウザアクション設定セクション */}
          {formData.notificationSettings.notificationType !== 'none' && (
            <div className="border-t pt-4">
              <div className="flex items-center justify-between mb-3">
                <h3 className="text-sm font-medium text-gray-700">ブラウザアクション</h3>
                <label className="flex items-center">
                  <input
                    type="checkbox"
                    checked={formData.browserActions.enabled}
                    onChange={(e) => handleBrowserActionToggle(e.target.checked)}
                    className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                  />
                  <span className="ml-2 text-sm text-gray-600">有効にする</span>
                </label>
              </div>
              
              <div className="text-sm text-gray-500 mb-3">
                通知時に指定したWebページを自動で開きます
              </div>
              
              {formData.browserActions.enabled && (
                <URLActionConfig
                  actions={browserActions || []}
                  onChange={handleBrowserActionsChange}
                  disabled={false}
                />
              )}
            </div>
          )}
          
          <div className="flex justify-end space-x-3 pt-4">
            <button
              type="button"
              onClick={handleClose}
              className="px-4 py-2 text-gray-600 hover:text-gray-800"
            >
              キャンセル
            </button>
            <button
              type="submit"
              className="btn-primary"
            >
              更新
            </button>
          </div>
        </form>
        </div>
      </div>
    </div>
  );
};