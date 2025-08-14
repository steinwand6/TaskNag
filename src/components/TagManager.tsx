import React, { useState, useEffect } from 'react';
import { Tag, CreateTagRequest, UpdateTagRequest } from '../types/Task';
import { useTaskStore } from '../stores/taskStore';
import { XMarkIcon, PlusIcon, PencilIcon, TrashIcon } from '@heroicons/react/24/outline';

interface TagManagerProps {
  isOpen: boolean;
  onClose: () => void;
}

const DEFAULT_COLORS = [
  '#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4', '#FFEAA7',
  '#DDA0DD', '#98D8C8', '#F7DC6F', '#BB8FCE', '#85C1E9',
  '#F8C471', '#82E0AA', '#F1948A', '#85C1E9', '#D2B4DE'
];

export const TagManager: React.FC<TagManagerProps> = ({ isOpen, onClose }) => {
  const { tags, loadTags, createTag, updateTag, deleteTag } = useTaskStore();
  const [isCreating, setIsCreating] = useState(false);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [formData, setFormData] = useState({ name: '', color: DEFAULT_COLORS[0] });

  useEffect(() => {
    if (isOpen) {
      loadTags();
    }
  }, [isOpen, loadTags]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!formData.name.trim()) return;

    try {
      if (editingId) {
        const updateData: UpdateTagRequest = {
          name: formData.name,
          color: formData.color,
        };
        await updateTag(editingId, updateData);
        setEditingId(null);
      } else {
        const createData: CreateTagRequest = {
          name: formData.name,
          color: formData.color,
        };
        await createTag(createData);
        setIsCreating(false);
      }
      setFormData({ name: '', color: DEFAULT_COLORS[0] });
    } catch (error) {
      console.error('Failed to save tag:', error);
    }
  };

  const handleEdit = (tag: Tag) => {
    setEditingId(tag.id);
    setFormData({ name: tag.name, color: tag.color });
    setIsCreating(false);
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('このタグを削除しますか？関連付けられたタスクからも削除されます。')) {
      try {
        await deleteTag(id);
      } catch (error) {
        console.error('Failed to delete tag:', error);
      }
    }
  };

  const handleCancel = () => {
    setIsCreating(false);
    setEditingId(null);
    setFormData({ name: '', color: DEFAULT_COLORS[0] });
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] overflow-hidden">
        <div className="flex items-center justify-between p-6 border-b">
          <h2 className="text-xl font-semibold">タグ管理</h2>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <XMarkIcon className="w-5 h-5" />
          </button>
        </div>

        <div className="p-6 overflow-y-auto max-h-[calc(90vh-120px)]">
          {/* 新規作成ボタン */}
          {!isCreating && !editingId && (
            <button
              onClick={() => setIsCreating(true)}
              className="w-full mb-4 p-3 border-2 border-dashed border-gray-300 rounded-lg hover:border-blue-400 hover:bg-blue-50 transition-colors flex items-center justify-center gap-2"
            >
              <PlusIcon className="w-5 h-5" />
              新しいタグを作成
            </button>
          )}

          {/* タグ作成/編集フォーム */}
          {(isCreating || editingId) && (
            <form onSubmit={handleSubmit} className="mb-6 p-4 bg-gray-50 rounded-lg">
              <div className="mb-4">
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  タグ名
                </label>
                <input
                  type="text"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  className="w-full p-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  placeholder="タグ名を入力"
                  required
                />
              </div>

              <div className="mb-4">
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  カラー
                </label>
                <div className="flex flex-wrap gap-2 mb-2">
                  {DEFAULT_COLORS.map((color) => (
                    <button
                      key={color}
                      type="button"
                      onClick={() => setFormData({ ...formData, color })}
                      className={`w-8 h-8 rounded-full border-2 ${
                        formData.color === color ? 'border-gray-800' : 'border-gray-300'
                      }`}
                      style={{ backgroundColor: color }}
                    />
                  ))}
                </div>
                <input
                  type="color"
                  value={formData.color}
                  onChange={(e) => setFormData({ ...formData, color: e.target.value })}
                  className="w-full h-10 border border-gray-300 rounded-md"
                />
              </div>

              <div className="flex gap-2">
                <button
                  type="submit"
                  className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors"
                >
                  {editingId ? '更新' : '作成'}
                </button>
                <button
                  type="button"
                  onClick={handleCancel}
                  className="px-4 py-2 bg-gray-300 text-gray-700 rounded-md hover:bg-gray-400 transition-colors"
                >
                  キャンセル
                </button>
              </div>
            </form>
          )}

          {/* タグ一覧 */}
          <div className="space-y-2">
            {tags.map((tag) => (
              <div
                key={tag.id}
                className="flex items-center justify-between p-3 bg-white border border-gray-200 rounded-lg hover:shadow-sm transition-shadow"
              >
                <div className="flex items-center gap-3">
                  <div
                    className="w-4 h-4 rounded-full border border-gray-300"
                    style={{ backgroundColor: tag.color }}
                  />
                  <span className="font-medium">{tag.name}</span>
                </div>
                <div className="flex gap-1">
                  <button
                    onClick={() => handleEdit(tag)}
                    className="p-1 hover:bg-gray-100 rounded transition-colors"
                    title="編集"
                  >
                    <PencilIcon className="w-4 h-4" />
                  </button>
                  <button
                    onClick={() => handleDelete(tag.id)}
                    className="p-1 hover:bg-red-100 rounded transition-colors text-red-600"
                    title="削除"
                  >
                    <TrashIcon className="w-4 h-4" />
                  </button>
                </div>
              </div>
            ))}
          </div>

          {tags.length === 0 && (
            <div className="text-center py-8 text-gray-500">
              まだタグがありません。新しいタグを作成してください。
            </div>
          )}
        </div>
      </div>
    </div>
  );
};