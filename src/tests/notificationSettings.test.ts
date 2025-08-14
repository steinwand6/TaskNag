import { TaskService } from '../services/taskService';
import { TaskNotificationSettings } from '../types/Task';

// モックデータ
const mockTaskWithNotifications = {
  id: 'test-id',
  title: 'Test Task',
  description: 'Test Description',
  status: 'todo',
  priority: 'medium',
  parentId: null,
  dueDate: null,
  completedAt: null,
  createdAt: '2025-08-14T00:00:00Z',
  updatedAt: '2025-08-14T00:00:00Z',
  progress: 0,
  // camelCase notification fields (as returned from backend)
  notificationType: 'recurring',
  notificationDaysBefore: null,
  notificationTime: '09:00',
  notificationDaysOfWeek: '[1,2,3,4,5]',
  notificationLevel: 2
};

const mockTaskWithoutNotifications = {
  id: 'test-id-2',
  title: 'Test Task 2',
  description: 'Test Description 2',
  status: 'todo',
  priority: 'medium',
  parentId: null,
  dueDate: null,
  completedAt: null,
  createdAt: '2025-08-14T00:00:00Z',
  updatedAt: '2025-08-14T00:00:00Z',
  progress: 0,
  // No notification settings
  notificationType: 'none',
  notificationDaysBefore: null,
  notificationTime: null,
  notificationDaysOfWeek: null,
  notificationLevel: 1
};

describe('Notification Settings Mapping', () => {
  test('should correctly map recurring notification settings', () => {
    // Access the private method via bracket notation for testing
    const mappedTask = (TaskService as any).mapTaskWithNotificationSettings(mockTaskWithNotifications);
    
    expect(mappedTask.notificationSettings).toBeDefined();
    expect(mappedTask.notificationSettings?.notificationType).toBe('recurring');
    expect(mappedTask.notificationSettings?.notificationTime).toBe('09:00');
    expect(mappedTask.notificationSettings?.daysOfWeek).toEqual([1, 2, 3, 4, 5]);
    expect(mappedTask.notificationSettings?.level).toBe(2);
    expect(mappedTask.notificationSettings?.daysBefore).toBeUndefined();
  });

  test('should handle tasks without notification settings', () => {
    const mappedTask = (TaskService as any).mapTaskWithNotificationSettings(mockTaskWithoutNotifications);
    
    expect(mappedTask.notificationSettings).toBeUndefined();
  });

  test('should correctly map due date based notification settings', () => {
    const mockTaskDueDateBased = {
      ...mockTaskWithNotifications,
      notificationType: 'due_date_based',
      notificationDaysBefore: 3,
      notificationTime: '10:00',
      notificationDaysOfWeek: null,
      notificationLevel: 3
    };

    const mappedTask = (TaskService as any).mapTaskWithNotificationSettings(mockTaskDueDateBased);
    
    expect(mappedTask.notificationSettings).toBeDefined();
    expect(mappedTask.notificationSettings?.notificationType).toBe('due_date_based');
    expect(mappedTask.notificationSettings?.daysBefore).toBe(3);
    expect(mappedTask.notificationSettings?.notificationTime).toBe('10:00');
    expect(mappedTask.notificationSettings?.level).toBe(3);
    expect(mappedTask.notificationSettings?.daysOfWeek).toBeUndefined();
  });

  test('should handle JSON parsing for daysOfWeek', () => {
    const mockTaskWithDaysOfWeek = {
      ...mockTaskWithNotifications,
      notificationDaysOfWeek: '[0,6]' // Weekend only
    };

    const mappedTask = (TaskService as any).mapTaskWithNotificationSettings(mockTaskWithDaysOfWeek);
    
    expect(mappedTask.notificationSettings?.daysOfWeek).toEqual([0, 6]);
  });

  test('should handle invalid JSON in daysOfWeek gracefully', () => {
    const mockTaskWithInvalidJSON = {
      ...mockTaskWithNotifications,
      notificationDaysOfWeek: 'invalid-json'
    };

    expect(() => {
      (TaskService as any).mapTaskWithNotificationSettings(mockTaskWithInvalidJSON);
    }).toThrow();
  });

  test('should convert date fields correctly', () => {
    const mockTaskWithDates = {
      ...mockTaskWithNotifications,
      dueDate: '2025-12-31T23:59:59Z',
      completedAt: '2025-08-14T12:00:00Z'
    };

    const mappedTask = (TaskService as any).mapTaskWithNotificationSettings(mockTaskWithDates);
    
    expect(mappedTask.dueDate).toBeInstanceOf(Date);
    expect(mappedTask.completedAt).toBeInstanceOf(Date);
    expect(mappedTask.createdAt).toBeInstanceOf(Date);
    expect(mappedTask.updatedAt).toBeInstanceOf(Date);
  });
});

// Integration test helper
export async function testNotificationSettingsIntegration() {
  console.log('🧪 Starting notification settings integration test...');
  
  try {
    // Test notification settings functionality
    const testSettings: TaskNotificationSettings = {
      notificationType: 'recurring',
      notificationTime: '09:00',
      daysOfWeek: [1, 2, 3, 4, 5],
      level: 2
    };

    console.log('✅ Test settings created:', testSettings);
    
    // Create a test task
    const testTask = await TaskService.createTask({
      title: 'Test Notification Task',
      description: 'Testing notification settings',
      status: 'todo',
      priority: 'medium',
      notificationSettings: testSettings
    });

    console.log('✅ Test task created:', testTask.id);

    // Retrieve the task and check if notification settings are preserved
    const retrievedTask = await TaskService.getTaskById(testTask.id);
    
    console.log('✅ Task retrieved, notification settings:', retrievedTask.notificationSettings);

    // Verify notification settings
    const settings = retrievedTask.notificationSettings;
    if (!settings) {
      throw new Error('Notification settings not found on retrieved task');
    }

    if (settings.notificationType !== 'recurring') {
      throw new Error(`Expected 'recurring', got '${settings.notificationType}'`);
    }

    if (settings.notificationTime !== '09:00') {
      throw new Error(`Expected '09:00', got '${settings.notificationTime}'`);
    }

    if (!settings.daysOfWeek || settings.daysOfWeek.length !== 5) {
      throw new Error(`Expected 5 days, got ${settings.daysOfWeek?.length || 0}`);
    }

    if (settings.level !== 2) {
      throw new Error(`Expected level 2, got ${settings.level}`);
    }

    // Update notification settings
    const updatedTask = await TaskService.updateTask(testTask.id, {
      notificationSettings: {
        notificationType: 'due_date_based',
        daysBefore: 3,
        notificationTime: '10:30',
        level: 3
      }
    });

    console.log('✅ Task updated, new notification settings:', updatedTask.notificationSettings);

    // Verify update
    const finalTask = await TaskService.getTaskById(testTask.id);
    const finalSettings = finalTask.notificationSettings;

    if (!finalSettings || finalSettings.notificationType !== 'due_date_based') {
      throw new Error('Failed to update notification settings');
    }

    // Clean up
    await TaskService.deleteTask(testTask.id);
    console.log('✅ Test task deleted');

    console.log('🎉 All notification settings tests passed!');
    return true;

  } catch (error) {
    console.error('❌ Notification settings test failed:', error);
    return false;
  }
}