import React from 'react';
import { Tag } from '../types/Task';

interface TagDisplayProps {
  tags: Tag[];
  maxDisplay?: number;
  size?: 'sm' | 'md' | 'lg';
  className?: string;
  onClick?: (tag: Tag) => void;
  onRemove?: (tagId: string) => void;
  showRemoveButton?: boolean;
}

export const TagDisplay: React.FC<TagDisplayProps> = ({ 
  tags, 
  maxDisplay = 3, 
  size = 'sm',
  className = '',
  onClick,
  onRemove,
  showRemoveButton = false,
}) => {
  if (!tags || tags.length === 0) {
    return null;
  }

  const displayTags = tags.slice(0, maxDisplay);
  const remainingCount = Math.max(0, tags.length - maxDisplay);

  const sizeClasses = {
    sm: 'px-2 py-0.5 text-xs',
    md: 'px-3 py-1 text-sm',
    lg: 'px-4 py-1.5 text-base',
  };

  const getTagClasses = (isClickable: boolean) => {
    const baseClasses = `inline-flex items-center gap-1 rounded-full font-medium border ${sizeClasses[size]}`;
    const interactiveClasses = isClickable ? 'cursor-pointer hover:opacity-80 transition-all duration-200 hover:scale-105' : '';
    return `${baseClasses} ${interactiveClasses}`;
  };

  return (
    <div className={`flex flex-wrap gap-1.5 ${className}`}>
      {displayTags.map((tag) => (
        <span
          key={tag.id}
          className={getTagClasses(!!onClick)}
          style={{
            backgroundColor: tag.color + '15', // 15% opacity for subtle background
            color: tag.color,
            borderColor: tag.color + '50', // 50% opacity for border
          }}
          onClick={onClick ? () => onClick(tag) : undefined}
          title={`タグ: ${tag.name}${onClick ? ' (クリックしてフィルタ)' : ''}`}
        >
          <span>{tag.name}</span>
          {showRemoveButton && onRemove && (
            <button
              className="ml-1 hover:bg-red-100 hover:text-red-600 rounded-full p-0.5 transition-colors"
              onClick={(e) => {
                e.stopPropagation();
                onRemove(tag.id);
              }}
              title="タグを削除"
            >
              <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          )}
        </span>
      ))}
      {remainingCount > 0 && (
        <span 
          className={`inline-flex items-center rounded-full bg-gray-100 text-gray-600 font-medium border border-gray-300 ${sizeClasses[size]}`}
          title={`他に${remainingCount}個のタグがあります`}
        >
          +{remainingCount}
        </span>
      )}
    </div>
  );
};