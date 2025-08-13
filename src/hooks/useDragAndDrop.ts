import React from 'react';
import { TaskStatus } from '../types/Task';

export const useDragAndDrop = (moveTask: (id: string, newStatus: TaskStatus) => Promise<void>) => {
  const [dragOverStatus, setDragOverStatus] = React.useState<TaskStatus | null>(null);
  const [draggingTaskId, setDraggingTaskId] = React.useState<string | null>(null);

  const handleDragStart = React.useCallback((taskId: string) => {
    setDraggingTaskId(taskId);
  }, []);

  const handleDragEnd = React.useCallback(() => {
    setDraggingTaskId(null);
    setDragOverStatus(null);
  }, []);

  const handleColumnMouseEnter = React.useCallback((status: TaskStatus) => {
    if (draggingTaskId) {
      setDragOverStatus(status);
    }
  }, [draggingTaskId]);

  const handleColumnMouseLeave = React.useCallback(() => {
    if (draggingTaskId) {
      setDragOverStatus(null);
    }
  }, [draggingTaskId]);

  const handleColumnClick = React.useCallback(async (status: TaskStatus) => {
    if (draggingTaskId) {
      await moveTask(draggingTaskId, status);
      setDraggingTaskId(null);
      setDragOverStatus(null);
    }
  }, [draggingTaskId, moveTask]);

  return {
    dragOverStatus,
    draggingTaskId,
    handleDragStart,
    handleDragEnd,
    handleColumnMouseEnter,
    handleColumnMouseLeave,
    handleColumnClick,
  };
};