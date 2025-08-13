import React from 'react';
import { TaskStatus } from '../types/Task';
import { DEFAULT_TASK_STATUS } from '../constants';

export const useModal = () => {
  const [isModalOpen, setIsModalOpen] = React.useState(false);
  const [modalInitialStatus, setModalInitialStatus] = React.useState<TaskStatus>(DEFAULT_TASK_STATUS);

  const openModal = React.useCallback((status?: TaskStatus) => {
    setModalInitialStatus(status || DEFAULT_TASK_STATUS);
    setIsModalOpen(true);
  }, []);

  const closeModal = React.useCallback(() => {
    setIsModalOpen(false);
  }, []);

  return {
    isModalOpen,
    modalInitialStatus,
    openModal,
    closeModal,
  };
};