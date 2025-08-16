import React, { useState, useCallback, useRef } from 'react';
import URLInput from './URLInput';
import {
  URLActionConfigProps,
  BrowserAction,
  BrowserActionFormData,
  BrowserActionFormErrors,
  URLValidationResult,
  BROWSER_ACTION_CONSTRAINTS,
  DragDropItem
} from '../types/BrowserAction';

const URLActionConfig: React.FC<URLActionConfigProps> = ({
  actions = [],
  onChange,
  maxActions = BROWSER_ACTION_CONSTRAINTS.MAX_ACTIONS,
  disabled = false
}) => {
  const [draggedItem, setDraggedItem] = useState<DragDropItem | null>(null);
  const [validationResults, setValidationResults] = useState<Record<string, URLValidationResult>>({});
  const [newAction, setNewAction] = useState<BrowserActionFormData>({
    label: '',
    url: '',
    enabled: true
  });
  const [errors, setErrors] = useState<BrowserActionFormErrors>({});
  const [showAddForm, setShowAddForm] = useState(false);
  const dragCounter = useRef(0);

  // Form validation
  const validateForm = useCallback((): boolean => {
    const newErrors: BrowserActionFormErrors = {};

    if (!newAction.label.trim()) {
      newErrors.label = 'ラベルは必須です';
    } else if (newAction.label.length > BROWSER_ACTION_CONSTRAINTS.MAX_LABEL_LENGTH) {
      newErrors.label = `ラベルは${BROWSER_ACTION_CONSTRAINTS.MAX_LABEL_LENGTH}文字以内で入力してください`;
    }

    if (!newAction.url.trim()) {
      newErrors.url = 'URLは必須です';
    }

    // Check for duplicate labels
    if (actions && actions.some(action => action.label.toLowerCase() === newAction.label.toLowerCase())) {
      newErrors.label = 'このラベルは既に使用されています';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  }, [newAction, actions]);

  // Add new action
  const handleAddAction = useCallback(() => {
    if (!validateForm()) return;

    const urlValidation = validationResults[newAction.url];
    if (!urlValidation?.isValid) {
      setErrors({ url: '有効なURLを入力してください' });
      return;
    }

    const newBrowserAction: BrowserAction = {
      id: crypto.randomUUID(),
      label: newAction.label.trim(),
      url: newAction.url.trim(),
      enabled: newAction.enabled,
      order: actions.length,
      createdAt: new Date()
    };

    onChange([...actions, newBrowserAction]);
    
    // Reset form
    setNewAction({ label: '', url: '', enabled: true });
    setErrors({});
    setShowAddForm(false);
    setValidationResults({});
  }, [newAction, actions, onChange, validateForm, validationResults]);

  // Remove action
  const handleRemoveAction = useCallback((id: string) => {
    const updatedActions = actions
      .filter(action => action.id !== id)
      .map((action, index) => ({ ...action, order: index }));
    onChange(updatedActions);
  }, [actions, onChange]);

  // Toggle action enabled state
  const handleToggleEnabled = useCallback((id: string) => {
    const updatedActions = actions.map(action =>
      action.id === id ? { ...action, enabled: !action.enabled } : action
    );
    onChange(updatedActions);
  }, [actions, onChange]);

  // Handle URL validation for form
  const handleUrlValidation = useCallback((result: URLValidationResult) => {
    setValidationResults(prev => ({
      ...prev,
      [newAction.url]: result
    }));
  }, [newAction.url]);

  // Drag and drop handlers
  const handleDragStart = useCallback((e: React.DragEvent, action: BrowserAction, index: number) => {
    if (disabled) return;
    
    const item: DragDropItem = { id: action.id, index };
    setDraggedItem(item);
    e.dataTransfer.effectAllowed = 'move';
    e.dataTransfer.setData('text/html', e.currentTarget.outerHTML);
    (e.currentTarget as HTMLElement).style.opacity = '0.5';
  }, [disabled]);

  const handleDragEnd = useCallback((e: React.DragEvent) => {
    (e.currentTarget as HTMLElement).style.opacity = '1';
    setDraggedItem(null);
    dragCounter.current = 0;
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
  }, []);

  const handleDragEnter = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    dragCounter.current++;
  }, []);

  const handleDragLeave = useCallback(() => {
    dragCounter.current--;
  }, []);

  const handleDrop = useCallback((e: React.DragEvent, dropIndex: number) => {
    e.preventDefault();
    dragCounter.current = 0;

    if (!draggedItem || draggedItem.index === dropIndex) return;

    const newActions = [...actions];
    const draggedAction = newActions[draggedItem.index];
    
    // Remove from old position
    newActions.splice(draggedItem.index, 1);
    
    // Insert at new position
    const insertIndex = draggedItem.index < dropIndex ? dropIndex - 1 : dropIndex;
    newActions.splice(insertIndex, 0, draggedAction);
    
    // Update order
    const reorderedActions = newActions.map((action, index) => ({
      ...action,
      order: index
    }));

    onChange(reorderedActions);
    setDraggedItem(null);
  }, [draggedItem, actions, onChange]);

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-medium text-gray-900">
          ブラウザアクション ({actions.length}/{maxActions})
        </h3>
        {actions.length < maxActions && (
          <button
            type="button"
            onClick={() => setShowAddForm(!showAddForm)}
            disabled={disabled}
            className={`
              px-3 py-1 text-sm rounded border transition-colors
              ${disabled
                ? 'bg-gray-100 text-gray-400 cursor-not-allowed'
                : 'bg-blue-50 text-blue-600 border-blue-200 hover:bg-blue-100'
              }
            `}
          >
            {showAddForm ? 'キャンセル' : '+ 追加'}
          </button>
        )}
      </div>

      {/* Add new action form */}
      {showAddForm && (
        <div className="p-4 border border-gray-200 rounded-lg bg-gray-50">
          <h4 className="text-sm font-medium text-gray-700 mb-3">新しいアクション</h4>
          
          <div className="space-y-3">
            {/* Label input */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                ラベル
              </label>
              <input
                type="text"
                value={newAction.label}
                onChange={(e) => setNewAction(prev => ({ ...prev, label: e.target.value }))}
                placeholder="例: プロジェクトボード"
                disabled={disabled}
                className={`
                  w-full px-3 py-2 border rounded-md transition-colors
                  ${errors.label
                    ? 'border-red-500 focus:border-red-500 focus:ring-red-200'
                    : 'border-gray-300 focus:border-blue-500 focus:ring-blue-200'
                  }
                  ${disabled ? 'bg-gray-100 cursor-not-allowed' : 'bg-white'}
                  focus:outline-none focus:ring-2
                `}
              />
              {errors.label && (
                <p className="text-red-600 text-sm mt-1">{errors.label}</p>
              )}
            </div>

            {/* URL input */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                URL
              </label>
              <URLInput
                value={newAction.url}
                onChange={(url) => setNewAction(prev => ({ ...prev, url }))}
                onValidation={handleUrlValidation}
                placeholder="https://example.com"
                disabled={disabled}
                error={errors.url}
                showPreview={false}
                showTestButton={true}
              />
            </div>

            {/* Enabled toggle */}
            <div className="flex items-center">
              <input
                type="checkbox"
                id="new-action-enabled"
                checked={newAction.enabled}
                onChange={(e) => setNewAction(prev => ({ ...prev, enabled: e.target.checked }))}
                disabled={disabled}
                className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
              />
              <label htmlFor="new-action-enabled" className="ml-2 text-sm text-gray-700">
                有効にする
              </label>
            </div>

            {/* Action buttons */}
            <div className="flex justify-end space-x-2 pt-2">
              <button
                type="button"
                onClick={() => {
                  setShowAddForm(false);
                  setNewAction({ label: '', url: '', enabled: true });
                  setErrors({});
                }}
                disabled={disabled}
                className="px-3 py-1 text-sm text-gray-600 border border-gray-300 rounded hover:bg-gray-50 transition-colors disabled:cursor-not-allowed"
              >
                キャンセル
              </button>
              <button
                type="button"
                onClick={handleAddAction}
                disabled={disabled || !newAction.label.trim() || !newAction.url.trim()}
                className={`
                  px-3 py-1 text-sm rounded transition-colors
                  ${disabled || !newAction.label.trim() || !newAction.url.trim()
                    ? 'bg-gray-100 text-gray-400 cursor-not-allowed'
                    : 'bg-blue-500 text-white hover:bg-blue-600'
                  }
                `}
              >
                追加
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Actions list */}
      {actions.length > 0 ? (
        <div className="space-y-2">
          {actions
            .sort((a, b) => a.order - b.order)
            .map((action, index) => (
              <div
                key={action.id}
                draggable={!disabled}
                onDragStart={(e) => handleDragStart(e, action, index)}
                onDragEnd={handleDragEnd}
                onDragOver={handleDragOver}
                onDragEnter={handleDragEnter}
                onDragLeave={handleDragLeave}
                onDrop={(e) => handleDrop(e, index)}
                className={`
                  p-3 border border-gray-200 rounded-lg bg-white transition-all
                  ${!disabled ? 'cursor-move hover:border-gray-300 hover:shadow-sm' : ''}
                  ${!action.enabled ? 'opacity-60' : ''}
                  ${draggedItem?.id === action.id ? 'opacity-50' : ''}
                `}
              >
                <div className="flex items-center justify-between">
                  {/* Drag handle and info */}
                  <div className="flex items-center space-x-3 flex-1">
                    {!disabled && (
                      <div className="cursor-move text-gray-400">
                        <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                          <path d="M7 2a2 2 0 1 0 0 4 2 2 0 0 0 0-4zM7 8a2 2 0 1 0 0 4 2 2 0 0 0 0-4zM7 14a2 2 0 1 0 0 4 2 2 0 0 0 0-4zM13 2a2 2 0 1 0 0 4 2 2 0 0 0 0-4zM13 8a2 2 0 1 0 0 4 2 2 0 0 0 0-4zM13 14a2 2 0 1 0 0 4 2 2 0 0 0 0-4z" />
                        </svg>
                      </div>
                    )}
                    
                    <div className="flex-1 min-w-0">
                      <h4 className="font-medium text-gray-900 truncate">{action.label}</h4>
                      <p className="text-sm text-gray-500 truncate">{action.url}</p>
                    </div>
                  </div>

                  {/* Controls */}
                  <div className="flex items-center space-x-2">
                    {/* Enabled toggle */}
                    <button
                      type="button"
                      onClick={() => handleToggleEnabled(action.id)}
                      disabled={disabled}
                      className={`
                        relative inline-flex h-6 w-11 items-center rounded-full transition-colors
                        ${action.enabled ? 'bg-blue-600' : 'bg-gray-200'}
                        ${disabled ? 'cursor-not-allowed opacity-50' : 'cursor-pointer'}
                      `}
                    >
                      <span
                        className={`
                          inline-block h-4 w-4 transform rounded-full bg-white transition-transform
                          ${action.enabled ? 'translate-x-6' : 'translate-x-1'}
                        `}
                      />
                    </button>

                    {/* Remove button */}
                    <button
                      type="button"
                      onClick={() => handleRemoveAction(action.id)}
                      disabled={disabled}
                      className={`
                        p-1 text-gray-400 hover:text-red-500 transition-colors
                        ${disabled ? 'cursor-not-allowed opacity-50' : ''}
                      `}
                    >
                      <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path fillRule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clipRule="evenodd" />
                      </svg>
                    </button>
                  </div>
                </div>
              </div>
            ))}
        </div>
      ) : (
        <div className="text-center py-8 border-2 border-dashed border-gray-200 rounded-lg">
          <svg className="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.746 0 3.332.477 4.5 1.253v13C19.832 18.477 18.246 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
          </svg>
          <h3 className="mt-2 text-sm font-medium text-gray-900">アクションなし</h3>
          <p className="mt-1 text-sm text-gray-500">
            通知時に開くWebページのアクションを追加してください
          </p>
        </div>
      )}

      {/* Limits info */}
      {actions.length >= maxActions && (
        <div className="text-sm text-orange-600 bg-orange-50 p-3 rounded-lg border border-orange-200">
          最大{maxActions}個までのアクションを設定できます
        </div>
      )}

      {/* Help text */}
      <div className="text-sm text-gray-500">
        <p>💡 <strong>ヒント:</strong></p>
        <ul className="mt-1 space-y-1 list-disc list-inside">
          <li>アクションはドラッグ&ドロップで並び替えできます</li>
          <li>通知時に有効なアクションのURLが順番に開かれます</li>
          <li>セキュリティのため、安全なURL（https://）のみ使用してください</li>
        </ul>
      </div>
    </div>
  );
};

export default URLActionConfig;