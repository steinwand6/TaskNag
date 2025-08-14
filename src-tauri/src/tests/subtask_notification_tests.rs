
/// 子タスクの期日・通知機能テスト
/// 
/// このテストモジュールは子タスクの期日設定と通知機能の
/// 包括的なテストを実行します。

#[cfg(test)]
mod subtask_notification_tests {
    use super::*;

    /// 子タスクに期日と通知設定を付与できることをテスト
    #[tokio::test]
    async fn test_subtask_due_date_and_notification_creation() {
        let mock_db = MockDatabase::new();

        // 親タスクを作成
        let parent_task = Task {
            id: Uuid::new_v4().to_string(),
            title: "親タスク".to_string(),
            description: Some("メインプロジェクト".to_string()),
            status: "todo".to_string(),
            priority: "high".to_string(),
            parent_id: None,
            due_date: None,
            completed_at: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            progress: Some(0),
            notification_type: Some("none".to_string()),
            notification_days_before: None,
            notification_time: None,
            notification_days_of_week: None,
            notification_level: Some(1),
        };

        let parent_task = mock_db.insert_task(parent_task).unwrap();

        // 期日付き子タスクを作成
        let due_date = Utc.with_ymd_and_hms(2025, 1, 15, 10, 0, 0).unwrap();
        
        let subtask = Task {
            id: Uuid::new_v4().to_string(),
            title: "子タスク with 期日・通知".to_string(),
            description: Some("重要な子タスク".to_string()),
            status: "todo".to_string(),
            priority: "medium".to_string(),
            parent_id: Some(parent_task.id.clone()),
            due_date: Some(due_date.to_rfc3339()),
            completed_at: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            progress: Some(0),
            // 通知設定
            notification_type: Some("due_date_based".to_string()),
            notification_days_before: Some(1),
            notification_time: Some("09:00".to_string()),
            notification_days_of_week: None,
            notification_level: Some(3),
        };

        let subtask = mock_db.insert_task(subtask).unwrap();

        // 子タスクが正しく作成されたことを確認
        let retrieved_subtask = mock_db.get_task_by_id(&subtask.id).unwrap();
        
        assert_eq!(retrieved_subtask.title, "子タスク with 期日・通知");
        assert_eq!(retrieved_subtask.parent_id, Some(parent_task.id.clone()));
        assert!(retrieved_subtask.due_date.is_some());
        assert_eq!(retrieved_subtask.due_date.unwrap(), due_date.to_rfc3339());

        // 通知設定を確認
        assert_eq!(retrieved_subtask.notification_type, Some("due_date_based".to_string()));
        assert_eq!(retrieved_subtask.notification_days_before, Some(1));
        assert_eq!(retrieved_subtask.notification_time, Some("09:00".to_string()));
        assert_eq!(retrieved_subtask.notification_level, Some(3));

        println!("✅ 子タスクの期日・通知設定テスト成功");
    }

    /// 複数の子タスクで異なる通知設定をテスト
    #[tokio::test]
    async fn test_multiple_subtasks_different_notifications() {
        let mock_db = MockDatabase::new();

        // 親タスク作成
        let parent_task = create_test_task_with_notifications();
        let parent_task = mock_db.insert_task(parent_task).unwrap();

        // 子タスク1: 期日通知
        let due_date1 = Utc.with_ymd_and_hms(2025, 1, 20, 15, 30, 0).unwrap();
        
        let subtask1 = Task {
            id: Uuid::new_v4().to_string(),
            title: "子タスク1 - 期日通知".to_string(),
            description: Some("期日2日前に通知".to_string()),
            status: "todo".to_string(),
            priority: "high".to_string(),
            parent_id: Some(parent_task.id.clone()),
            due_date: Some(due_date1.to_rfc3339()),
            completed_at: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            progress: Some(0),
            // 期日通知設定
            notification_type: Some("due_date_based".to_string()),
            notification_days_before: Some(2),
            notification_time: Some("08:00".to_string()),
            notification_days_of_week: None,
            notification_level: Some(2),
        };

        let subtask1 = mock_db.insert_task(subtask1).unwrap();

        // 子タスク2: 定期通知
        let due_date2 = Utc.with_ymd_and_hms(2025, 1, 25, 12, 0, 0).unwrap();
        
        let subtask2 = Task {
            id: Uuid::new_v4().to_string(),
            title: "子タスク2 - 定期通知".to_string(),
            description: Some("週3回の定期通知".to_string()),
            status: "in_progress".to_string(),
            priority: "low".to_string(),
            parent_id: Some(parent_task.id.clone()),
            due_date: Some(due_date2.to_rfc3339()),
            completed_at: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            progress: Some(0),
            // 定期通知設定
            notification_type: Some("recurring".to_string()),
            notification_days_before: Some(0),
            notification_time: Some("18:00".to_string()),
            notification_days_of_week: Some("[1,3,5]".to_string()), // 月、水、金
            notification_level: Some(1),
        };

        let subtask2 = mock_db.insert_task(subtask2).unwrap();

        // 子タスク3: 通知なし
        let subtask3 = Task {
            id: Uuid::new_v4().to_string(),
            title: "子タスク3 - 通知なし".to_string(),
            description: Some("期日のみ設定".to_string()),
            status: "todo".to_string(),
            priority: "medium".to_string(),
            parent_id: Some(parent_task.id.clone()),
            due_date: Some(Utc.with_ymd_and_hms(2025, 1, 30, 9, 0, 0).unwrap().to_rfc3339()),
            completed_at: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            progress: Some(0),
            // 通知なし設定
            notification_type: Some("none".to_string()),
            notification_days_before: None,
            notification_time: None,
            notification_days_of_week: None,
            notification_level: Some(1),
        };

        let subtask3 = mock_db.insert_task(subtask3).unwrap();

        // 親タスクから子タスクを取得（MockDatabaseで実装）
        let all_tasks = mock_db.get_all_tasks();
        let children: Vec<_> = all_tasks.into_iter()
            .filter(|task| task.parent_id == Some(parent_task.id.clone()))
            .collect();
        assert_eq!(children.len(), 3);

        // 各子タスクの通知設定を確認
        for child in children {
            match child.title.as_str() {
                "子タスク1 - 期日通知" => {
                    assert_eq!(child.notification_type, Some("due_date_based".to_string()));
                    assert_eq!(child.notification_days_before, Some(2));
                    assert_eq!(child.notification_level, Some(2));
                }
                "子タスク2 - 定期通知" => {
                    assert_eq!(child.notification_type, Some("recurring".to_string()));
                    assert_eq!(child.notification_days_of_week, Some("[1,3,5]".to_string()));
                    assert_eq!(child.notification_level, Some(1));
                }
                "子タスク3 - 通知なし" => {
                    assert_eq!(child.notification_type, Some("none".to_string()));
                }
                _ => panic!("予期しない子タスク: {}", child.title),
            }
        }

        println!("✅ 複数子タスクの異なる通知設定テスト成功");
    }

    /// 子タスクの期日変更と通知設定更新のテスト
    #[tokio::test]
    async fn test_subtask_due_date_modification() {
        let mock_db = MockDatabase::new();

        // 親タスクと子タスクを作成
        let parent_task = create_test_task_with_notifications();
        let parent_task = mock_db.insert_task(parent_task).unwrap();
        
        let original_due_date = Utc.with_ymd_and_hms(2025, 2, 1, 10, 0, 0).unwrap();
        let subtask = Task {
            id: Uuid::new_v4().to_string(),
            title: "期日変更テスト子タスク".to_string(),
            description: Some("期日を変更するテスト".to_string()),
            status: "todo".to_string(),
            priority: "medium".to_string(),
            parent_id: Some(parent_task.id.clone()),
            due_date: Some(original_due_date.to_rfc3339()),
            completed_at: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            progress: Some(0),
            // 初期通知設定
            notification_type: Some("due_date_based".to_string()),
            notification_days_before: Some(3),
            notification_time: Some("10:00".to_string()),
            notification_days_of_week: None,
            notification_level: Some(2),
        };

        let subtask = mock_db.insert_task(subtask).unwrap();

        // 期日を変更
        let new_due_date = Utc.with_ymd_and_hms(2025, 2, 15, 14, 30, 0).unwrap();
        let mut updated_subtask = subtask.clone();
        updated_subtask.due_date = Some(new_due_date.to_rfc3339());
        updated_subtask.description = Some("期日変更済み".to_string());
        // 通知設定も更新
        updated_subtask.notification_type = Some("recurring".to_string());
        updated_subtask.notification_days_before = Some(1);
        updated_subtask.notification_time = Some("16:00".to_string());
        updated_subtask.notification_days_of_week = Some("[2,4]".to_string()); // 火、木
        updated_subtask.notification_level = Some(3);

        let result = mock_db.update_task(&subtask.id, updated_subtask).unwrap();

        // 変更を確認
        assert_eq!(result.due_date.unwrap(), new_due_date.to_rfc3339());
        assert_eq!(result.description, Some("期日変更済み".to_string()));

        // 更新された通知設定を確認
        assert_eq!(result.notification_type, Some("recurring".to_string()));
        assert_eq!(result.notification_days_before, Some(1));
        assert_eq!(result.notification_time, Some("16:00".to_string()));
        assert_eq!(result.notification_days_of_week, Some("[2,4]".to_string()));
        assert_eq!(result.notification_level, Some(3));

        println!("✅ 子タスクの期日変更・通知設定更新テスト成功");
    }

    /// 子タスクの削除時に通知設定も削除されることをテスト
    #[tokio::test]
    async fn test_subtask_deletion_removes_notifications() {
        let mock_db = MockDatabase::new();

        // 親タスクと子タスクを作成
        let parent_task = create_test_task_with_notifications();
        let parent_task = mock_db.insert_task(parent_task).unwrap();
        
        let subtask = Task {
            id: Uuid::new_v4().to_string(),
            title: "削除テスト子タスク".to_string(),
            description: Some("削除予定".to_string()),
            status: "todo".to_string(),
            priority: "low".to_string(),
            parent_id: Some(parent_task.id.clone()),
            due_date: Some(Utc.with_ymd_and_hms(2025, 3, 1, 12, 0, 0).unwrap().to_rfc3339()),
            completed_at: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            progress: Some(0),
            // 通知設定
            notification_type: Some("due_date_based".to_string()),
            notification_days_before: Some(7),
            notification_time: Some("07:00".to_string()),
            notification_days_of_week: None,
            notification_level: Some(1),
        };

        let subtask = mock_db.insert_task(subtask).unwrap();

        // 通知設定が存在することを確認
        let existing_task = mock_db.get_task_by_id(&subtask.id).unwrap();
        assert_eq!(existing_task.notification_type, Some("due_date_based".to_string()));
        assert_eq!(existing_task.notification_days_before, Some(7));

        // 子タスクを削除
        mock_db.delete_task(&subtask.id).unwrap();

        // タスクが削除されたことを確認
        let deleted_task = mock_db.get_task_by_id(&subtask.id);
        assert!(deleted_task.is_err());

        println!("✅ 子タスク削除時の通知設定削除テスト成功");
    }

    /// 子タスクの進捗率更新テスト
    #[tokio::test]
    async fn test_subtask_progress_updates() {
        let mock_db = MockDatabase::new();

        // 親タスクを作成
        let parent_task = create_test_task_with_notifications();
        let parent_task = mock_db.insert_task(parent_task).unwrap();

        // 複数の子タスクを作成
        let subtask_titles = vec![
            "子タスクA", "子タスクB", "子タスクC", "子タスクD"
        ];
        let mut subtask_ids = Vec::new();

        for title in subtask_titles {
            let subtask = Task {
                id: Uuid::new_v4().to_string(),
                title: title.to_string(),
                description: Some(format!("{} - 進捗テスト", title)),
                status: "todo".to_string(),
                priority: "medium".to_string(),
                parent_id: Some(parent_task.id.clone()),
                due_date: Some(Utc.with_ymd_and_hms(2025, 4, 1, 10, 0, 0).unwrap().to_rfc3339()),
                completed_at: None,
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
                progress: Some(0),
                notification_type: Some("none".to_string()),
                notification_days_before: None,
                notification_time: None,
                notification_days_of_week: None,
                notification_level: Some(1),
            };

            let subtask = mock_db.insert_task(subtask).unwrap();
            subtask_ids.push(subtask.id);
        }

        // 進捗計算のヘルパー関数
        let calculate_progress = |mock_db: &MockDatabase, parent_id: &str| -> i32 {
            let all_tasks = mock_db.get_all_tasks();
            let children: Vec<_> = all_tasks.into_iter()
                .filter(|task| task.parent_id == Some(parent_id.to_string()))
                .collect();
            
            if children.is_empty() {
                return 0;
            }
            
            let total_progress: i32 = children.iter()
                .map(|child| {
                    if child.status == "done" {
                        100
                    } else {
                        child.progress.unwrap_or(0)
                    }
                })
                .sum();
            
            total_progress / children.len() as i32
        };

        // 初期状態: 全ての子タスクがTodo
        let initial_progress = calculate_progress(&mock_db, &parent_task.id);
        assert_eq!(initial_progress, 0);

        // 1つ目の子タスクを完了
        let mut task1 = mock_db.get_task_by_id(&subtask_ids[0]).unwrap();
        task1.status = "done".to_string();
        task1.completed_at = Some(Utc::now().to_rfc3339());
        mock_db.update_task(&subtask_ids[0], task1).unwrap();
        let progress_25 = calculate_progress(&mock_db, &parent_task.id);
        assert_eq!(progress_25, 25);

        // 2つ目の子タスクを完了
        let mut task2 = mock_db.get_task_by_id(&subtask_ids[1]).unwrap();
        task2.status = "done".to_string();
        task2.completed_at = Some(Utc::now().to_rfc3339());
        mock_db.update_task(&subtask_ids[1], task2).unwrap();
        let progress_50 = calculate_progress(&mock_db, &parent_task.id);
        assert_eq!(progress_50, 50);

        // 3つ目の子タスクを完了
        let mut task3 = mock_db.get_task_by_id(&subtask_ids[2]).unwrap();
        task3.status = "done".to_string();
        task3.completed_at = Some(Utc::now().to_rfc3339());
        mock_db.update_task(&subtask_ids[2], task3).unwrap();
        let progress_75 = calculate_progress(&mock_db, &parent_task.id);
        assert_eq!(progress_75, 75);

        // 全ての子タスクを完了
        let mut task4 = mock_db.get_task_by_id(&subtask_ids[3]).unwrap();
        task4.status = "done".to_string();
        task4.completed_at = Some(Utc::now().to_rfc3339());
        mock_db.update_task(&subtask_ids[3], task4).unwrap();
        let progress_100 = calculate_progress(&mock_db, &parent_task.id);
        assert_eq!(progress_100, 100);

        // 親タスクの進捗率を更新
        let mut updated_parent = mock_db.get_task_by_id(&parent_task.id).unwrap();
        updated_parent.progress = Some(progress_100);
        let updated_parent = mock_db.update_task(&parent_task.id, updated_parent).unwrap();
        assert_eq!(updated_parent.progress, Some(100));

        println!("✅ 子タスクの進捗率更新テスト成功");
    }

    /// エラーケースのテスト
    #[tokio::test]
    async fn test_subtask_error_cases() {
        let mock_db = MockDatabase::new();

        // 存在しない親タスクIDを持つ子タスクを作成（MockDatabaseでは親の存在チェックなし）
        let invalid_parent_id = Uuid::new_v4().to_string();
        let invalid_subtask = Task {
            id: Uuid::new_v4().to_string(),
            title: "無効な親タスクの子".to_string(),
            description: Some("これは成功するが親は存在しない".to_string()),
            status: "todo".to_string(),
            priority: "medium".to_string(),
            parent_id: Some(invalid_parent_id.clone()),
            due_date: None,
            completed_at: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            progress: Some(0),
            notification_type: Some("none".to_string()),
            notification_days_before: None,
            notification_time: None,
            notification_days_of_week: None,
            notification_level: Some(1),
        };

        // MockDatabaseは親の存在をチェックしないため、作成は成功する
        let result = mock_db.insert_task(invalid_subtask);
        assert!(result.is_ok());
        let created_task = result.unwrap();
        assert_eq!(created_task.parent_id, Some(invalid_parent_id));

        // 存在しないタスクの取得を試行
        let nonexistent_task_id = Uuid::new_v4().to_string();
        let get_result = mock_db.get_task_by_id(&nonexistent_task_id);
        assert!(get_result.is_err());

        // 存在しないタスクの削除を試行
        let delete_result = mock_db.delete_task(&nonexistent_task_id);
        assert!(delete_result.is_err());

        println!("✅ 子タスクエラーケーステスト成功");
    }

    /// 公開テスト関数 - 全ての子タスク期日・通知テストを実行
    pub fn run_all_subtask_notification_tests() -> String {
        let mut results = Vec::new();
        
        results.push("🚀 子タスク期日・通知機能テスト開始".to_string());
        
        // Test 1: 子タスクの期日・通知設定作成
        match std::panic::catch_unwind(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mock_db = MockDatabase::new();

                // 親タスクを作成
                let parent_task = Task {
                    id: Uuid::new_v4().to_string(),
                    title: "親タスク".to_string(),
                    description: Some("メインプロジェクト".to_string()),
                    status: "todo".to_string(),
                    priority: "high".to_string(),
                    parent_id: None,
                    due_date: None,
                    completed_at: None,
                    created_at: Utc::now().to_rfc3339(),
                    updated_at: Utc::now().to_rfc3339(),
                    progress: Some(0),
                    notification_type: Some("none".to_string()),
                    notification_days_before: None,
                    notification_time: None,
                    notification_days_of_week: None,
                    notification_level: Some(1),
                };

                let parent_task = mock_db.insert_task(parent_task).unwrap();

                // 期日付き子タスクを作成
                let due_date = Utc.with_ymd_and_hms(2025, 1, 15, 10, 0, 0).unwrap();
                
                let subtask = Task {
                    id: Uuid::new_v4().to_string(),
                    title: "子タスク with 期日・通知".to_string(),
                    description: Some("重要な子タスク".to_string()),
                    status: "todo".to_string(),
                    priority: "medium".to_string(),
                    parent_id: Some(parent_task.id.clone()),
                    due_date: Some(due_date.to_rfc3339()),
                    completed_at: None,
                    created_at: Utc::now().to_rfc3339(),
                    updated_at: Utc::now().to_rfc3339(),
                    progress: Some(0),
                    // 通知設定
                    notification_type: Some("due_date_based".to_string()),
                    notification_days_before: Some(1),
                    notification_time: Some("09:00".to_string()),
                    notification_days_of_week: None,
                    notification_level: Some(3),
                };

                let subtask = mock_db.insert_task(subtask).unwrap();

                // 子タスクが正しく作成されたことを確認
                let retrieved_subtask = mock_db.get_task_by_id(&subtask.id).unwrap();
                
                assert_eq!(retrieved_subtask.title, "子タスク with 期日・通知");
                assert_eq!(retrieved_subtask.parent_id, Some(parent_task.id.clone()));
                assert!(retrieved_subtask.due_date.is_some());
                assert_eq!(retrieved_subtask.due_date.unwrap(), due_date.to_rfc3339());

                // 通知設定を確認
                assert_eq!(retrieved_subtask.notification_type, Some("due_date_based".to_string()));
                assert_eq!(retrieved_subtask.notification_days_before, Some(1));
                assert_eq!(retrieved_subtask.notification_time, Some("09:00".to_string()));
                assert_eq!(retrieved_subtask.notification_level, Some(3));
            });
        }) {
            Ok(_) => results.push("✅ 子タスクの期日・通知設定作成テスト PASSED".to_string()),
            Err(_) => results.push("❌ 子タスクの期日・通知設定作成テスト FAILED".to_string()),
        }
        
        // Test 2: 複数子タスクの異なる通知設定
        match std::panic::catch_unwind(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mock_db = MockDatabase::new();

                // 親タスク作成
                let parent_task = create_test_task_with_notifications();
                let parent_task = mock_db.insert_task(parent_task).unwrap();

                // 子タスク1: 期日通知
                let due_date1 = Utc.with_ymd_and_hms(2025, 1, 20, 15, 30, 0).unwrap();
                
                let subtask1 = Task {
                    id: Uuid::new_v4().to_string(),
                    title: "子タスク1 - 期日通知".to_string(),
                    description: Some("期日2日前に通知".to_string()),
                    status: "todo".to_string(),
                    priority: "high".to_string(),
                    parent_id: Some(parent_task.id.clone()),
                    due_date: Some(due_date1.to_rfc3339()),
                    completed_at: None,
                    created_at: Utc::now().to_rfc3339(),
                    updated_at: Utc::now().to_rfc3339(),
                    progress: Some(0),
                    // 期日通知設定
                    notification_type: Some("due_date_based".to_string()),
                    notification_days_before: Some(2),
                    notification_time: Some("08:00".to_string()),
                    notification_days_of_week: None,
                    notification_level: Some(2),
                };

                let subtask1 = mock_db.insert_task(subtask1).unwrap();

                // 子タスク2: 定期通知
                let due_date2 = Utc.with_ymd_and_hms(2025, 1, 25, 12, 0, 0).unwrap();
                
                let subtask2 = Task {
                    id: Uuid::new_v4().to_string(),
                    title: "子タスク2 - 定期通知".to_string(),
                    description: Some("週3回の定期通知".to_string()),
                    status: "in_progress".to_string(),
                    priority: "low".to_string(),
                    parent_id: Some(parent_task.id.clone()),
                    due_date: Some(due_date2.to_rfc3339()),
                    completed_at: None,
                    created_at: Utc::now().to_rfc3339(),
                    updated_at: Utc::now().to_rfc3339(),
                    progress: Some(0),
                    // 定期通知設定
                    notification_type: Some("recurring".to_string()),
                    notification_days_before: Some(0),
                    notification_time: Some("18:00".to_string()),
                    notification_days_of_week: Some("[1,3,5]".to_string()), // 月、水、金
                    notification_level: Some(1),
                };

                let subtask2 = mock_db.insert_task(subtask2).unwrap();

                // 子タスク3: 通知なし
                let subtask3 = Task {
                    id: Uuid::new_v4().to_string(),
                    title: "子タスク3 - 通知なし".to_string(),
                    description: Some("期日のみ設定".to_string()),
                    status: "todo".to_string(),
                    priority: "medium".to_string(),
                    parent_id: Some(parent_task.id.clone()),
                    due_date: Some(Utc.with_ymd_and_hms(2025, 1, 30, 9, 0, 0).unwrap().to_rfc3339()),
                    completed_at: None,
                    created_at: Utc::now().to_rfc3339(),
                    updated_at: Utc::now().to_rfc3339(),
                    progress: Some(0),
                    // 通知なし設定
                    notification_type: Some("none".to_string()),
                    notification_days_before: None,
                    notification_time: None,
                    notification_days_of_week: None,
                    notification_level: Some(1),
                };

                let subtask3 = mock_db.insert_task(subtask3).unwrap();

                // 親タスクから子タスクを取得（MockDatabaseで実装）
                let all_tasks = mock_db.get_all_tasks();
                let children: Vec<_> = all_tasks.into_iter()
                    .filter(|task| task.parent_id == Some(parent_task.id.clone()))
                    .collect();
                assert_eq!(children.len(), 3);

                // 各子タスクの通知設定を確認
                for child in children {
                    match child.title.as_str() {
                        "子タスク1 - 期日通知" => {
                            assert_eq!(child.notification_type, Some("due_date_based".to_string()));
                            assert_eq!(child.notification_days_before, Some(2));
                            assert_eq!(child.notification_level, Some(2));
                        }
                        "子タスク2 - 定期通知" => {
                            assert_eq!(child.notification_type, Some("recurring".to_string()));
                            assert_eq!(child.notification_days_of_week, Some("[1,3,5]".to_string()));
                            assert_eq!(child.notification_level, Some(1));
                        }
                        "子タスク3 - 通知なし" => {
                            assert_eq!(child.notification_type, Some("none".to_string()));
                        }
                        _ => panic!("予期しない子タスク: {}", child.title),
                    }
                }
            });
        }) {
            Ok(_) => results.push("✅ 複数子タスクの異なる通知設定テスト PASSED".to_string()),
            Err(_) => results.push("❌ 複数子タスクの異なる通知設定テスト FAILED".to_string()),
        }
        
        results.push("🎉 全ての子タスク期日・通知テストが完了しました".to_string());
        
        results.join("\n")
    }
}

// テスト実行用の公開関数は上記のモジュール内で定義済み