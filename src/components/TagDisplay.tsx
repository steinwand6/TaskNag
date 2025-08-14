import React from 'react';
import { Tag } from '../types/Task';

interface TagDisplayProps {
  tags: Tag[];
  maxDisplay?: number;
  size?: 'sm' | 'md' | 'lg';
  className?: string;
}

export const TagDisplay: React.FC<TagDisplayProps> = ({ 
  tags, 
  maxDisplay = 3, 
  size = 'sm',
  className = '' 
}) => {
  if (!tags || tags.length === 0) {
    return null;
  }

  const displayTags = tags.slice(0, maxDisplay);
  const remainingCount = tags.length - maxDisplay;

  const sizeClasses = {
    sm: 'px-2 py-0.5 text-xs',
    md: 'px-3 py-1 text-sm',
    lg: 'px-4 py-1.5 text-base',
  };

  return (
    <div className={`flex flex-wrap gap-1 ${className}`}>
      {displayTags.map((tag) => (
        <span
          key={tag.id}
          className={`inline-flex items-center rounded-full font-medium ${sizeClasses[size]}`}
          style={{
            backgroundColor: tag.color + '20', // 20% opacity
            color: tag.color,
            borderWidth: '1px',
            borderColor: tag.color + '40', // 40% opacity
          }}
        >
          {tag.name}
        </span>
      ))}
      {remainingCount > 0 && (
        <span className={`inline-flex items-center rounded-full bg-gray-100 text-gray-600 font-medium ${sizeClasses[size]}`}>
          +{remainingCount}
        </span>
      )}
    </div>
  );
};