import { BrowserAction, BrowserActionSettings } from '../types/BrowserAction';

describe('Browser Action Data Structure Tests', () => {
  test('should create valid BrowserAction object', () => {
    const action: BrowserAction = {
      id: 'test-action-1',
      label: 'Test Action',
      url: 'https://example.com',
      enabled: true,
      order: 1,
      createdAt: new Date('2024-01-01T00:00:00Z')
    };

    expect(action.id).toBe('test-action-1');
    expect(action.label).toBe('Test Action');
    expect(action.url).toBe('https://example.com');
    expect(action.enabled).toBe(true);
    expect(action.order).toBe(1);
    expect(action.createdAt).toEqual(new Date('2024-01-01T00:00:00Z'));
  });

  test('should create valid BrowserActionSettings object', () => {
    const actions: BrowserAction[] = [
      {
        id: 'action-1',
        label: 'Google',
        url: 'https://google.com',
        enabled: true,
        order: 1,
        createdAt: new Date()
      },
      {
        id: 'action-2',
        label: 'GitHub',
        url: 'https://github.com',
        enabled: true,
        order: 2,
        createdAt: new Date()
      }
    ];

    const settings: BrowserActionSettings = {
      enabled: true,
      actions: actions
    };

    expect(settings.enabled).toBe(true);
    expect(settings.actions).toHaveLength(2);
    expect(settings.actions[0].label).toBe('Google');
    expect(settings.actions[1].label).toBe('GitHub');
  });

  test('should serialize and deserialize BrowserActionSettings correctly', () => {
    const originalSettings: BrowserActionSettings = {
      enabled: true,
      actions: [
        {
          id: 'action-1',
          label: 'Test Action',
          url: 'https://example.com',
          enabled: true,
          order: 1,
          createdAt: new Date('2024-01-01T00:00:00Z')
        }
      ]
    };

    // Serialize
    const serialized = JSON.stringify(originalSettings);
    expect(serialized).toContain('Test Action');
    expect(serialized).toContain('https://example.com');

    // Deserialize
    const deserialized = JSON.parse(serialized) as BrowserActionSettings;
    expect(deserialized.enabled).toBe(true);
    expect(deserialized.actions).toHaveLength(1);
    expect(deserialized.actions[0].label).toBe('Test Action');
    expect(deserialized.actions[0].url).toBe('https://example.com');
    expect(deserialized.actions[0].enabled).toBe(true);
    
    // Note: Date is serialized as string, so we need to check the string value
    expect(deserialized.actions[0].createdAt).toBe('2024-01-01T00:00:00.000Z');
  });

  test('should handle empty browser actions', () => {
    const settings: BrowserActionSettings = {
      enabled: false,
      actions: []
    };

    expect(settings.enabled).toBe(false);
    expect(settings.actions).toHaveLength(0);

    const serialized = JSON.stringify(settings);
    const deserialized = JSON.parse(serialized) as BrowserActionSettings;
    
    expect(deserialized.enabled).toBe(false);
    expect(deserialized.actions).toHaveLength(0);
  });

  test('should preserve action order during serialization', () => {
    const settings: BrowserActionSettings = {
      enabled: true,
      actions: [
        {
          id: 'action-3',
          label: 'Third',
          url: 'https://third.com',
          enabled: true,
          order: 3,
          createdAt: new Date()
        },
        {
          id: 'action-1',
          label: 'First',
          url: 'https://first.com',
          enabled: true,
          order: 1,
          createdAt: new Date()
        },
        {
          id: 'action-2',
          label: 'Second',
          url: 'https://second.com',
          enabled: true,
          order: 2,
          createdAt: new Date()
        }
      ]
    };

    const serialized = JSON.stringify(settings);
    const deserialized = JSON.parse(serialized) as BrowserActionSettings;

    // Original order should be preserved
    expect(deserialized.actions[0].label).toBe('Third');
    expect(deserialized.actions[1].label).toBe('First');
    expect(deserialized.actions[2].label).toBe('Second');

    // But we can sort by order property
    const sorted = deserialized.actions.sort((a, b) => a.order - b.order);
    expect(sorted[0].label).toBe('First');
    expect(sorted[1].label).toBe('Second');
    expect(sorted[2].label).toBe('Third');
  });
});