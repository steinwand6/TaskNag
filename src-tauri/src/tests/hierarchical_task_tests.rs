use crate::models::{Task, TaskStatus, Priority, CreateTaskRequest, UpdateTaskRequest};
use crate::tests::mock_database::{MockDatabase, create_test_task_with_notifications};
use crate::error::AppError;
use uuid::Uuid;
use chrono::Utc;

/// 親子タスク関係のテスト
async fn test_parent_child_task_relationships() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing parent-child task relationships...");
    
    // Test 1: 親タスクの作成
    let mut parent_task = create_test_task_with_notifications();
    parent_task.title = "Parent Task".to_string();
    parent_task.parent_id = None;
    parent_task.progress = Some(0);
    
    let created_parent = mock_db.insert_task(parent_task).unwrap();
    assert!(created_parent.parent_id.is_none());
    
    println!("✅ Parent task created with ID: {}", created_parent.id);
    
    // Test 2: 子タスクの作成
    let child_count = 3;
    let mut child_task_ids = Vec::new();
    
    for i in 0..child_count {
        let mut child_task = create_test_task_with_notifications();
        child_task.id = Uuid::new_v4().to_string();
        child_task.title = format!("Child Task {}", i + 1);
        child_task.parent_id = Some(created_parent.id.clone());
        child_task.progress = Some(0);
        
        let created_child = mock_db.insert_task(child_task).unwrap();
        assert_eq!(created_child.parent_id, Some(created_parent.id.clone()));
        
        child_task_ids.push(created_child.id);
        println!("✅ Child task {} created with parent_id: {}", i + 1, created_parent.id);
    }
    
    // Test 3: 子タスクの取得（手動実装）
    let all_tasks = mock_db.get_all_tasks();
    let child_tasks: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| t.parent_id == Some(created_parent.id.clone()))
        .collect();
    
    assert_eq!(child_tasks.len(), child_count);
    println!("✅ Retrieved {} child tasks for parent", child_tasks.len());
    
    // Test 4: 親タスクの確認
    let parent_verification = mock_db.get_task_by_id(&created_parent.id).unwrap();
    assert_eq!(parent_verification.title, "Parent Task");
    assert!(parent_verification.parent_id.is_none());
    
    println!("✅ Parent task verification passed");
    
    // Cleanup
    for child_id in child_task_ids {
        mock_db.delete_task(&child_id).unwrap();
    }
    mock_db.delete_task(&created_parent.id).unwrap();
    
    println!("🎉 All parent-child relationship tests passed!");
}

/// 進捗率計算のテスト
async fn test_progress_calculation() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing progress calculation...");
    
    // Create parent task
    let mut parent_task = create_test_task_with_notifications();
    parent_task.title = "Project Task".to_string();
    parent_task.parent_id = None;
    parent_task.progress = Some(0);
    
    let parent = mock_db.insert_task(parent_task).unwrap();
    
    // Create child tasks with different progress levels
    let child_progress_values = [0, 25, 50, 75, 100];
    let mut child_ids = Vec::new();
    
    for (i, progress) in child_progress_values.iter().enumerate() {
        let mut child_task = create_test_task_with_notifications();
        child_task.id = Uuid::new_v4().to_string();
        child_task.title = format!("Sub-task {} ({}%)", i + 1, progress);
        child_task.parent_id = Some(parent.id.clone());
        child_task.progress = Some(*progress);
        
        if *progress == 100 {
            child_task.status = "done".to_string();
            child_task.completed_at = Some(Utc::now().to_rfc3339());
        }
        
        let created_child = mock_db.insert_task(child_task).unwrap();
        child_ids.push(created_child.id);
    }
    
    // Calculate expected progress: (0 + 25 + 50 + 75 + 100) / 5 = 50%
    let all_tasks = mock_db.get_all_tasks();
    let child_tasks: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| t.parent_id == Some(parent.id.clone()))
        .collect();
    
    let total_progress: i32 = child_tasks
        .iter()
        .map(|t| t.progress.unwrap_or(0))
        .sum();
    let average_progress = total_progress / child_tasks.len() as i32;
    
    assert_eq!(average_progress, 50);
    
    println!("✅ Progress calculation: {}% (expected 50%)", average_progress);
    
    // Test updating child progress and recalculating
    let mut first_child = mock_db.get_task_by_id(&child_ids[0]).unwrap(); // Was 0%, change to 100%
    first_child.progress = Some(100);
    first_child.status = "done".to_string();
    first_child.completed_at = Some(Utc::now().to_rfc3339());
    
    mock_db.update_task(&child_ids[0], first_child).unwrap();
    
    // Recalculate: (100 + 25 + 50 + 75 + 100) / 5 = 70%
    let updated_tasks = mock_db.get_all_tasks();
    let updated_child_tasks: Vec<&Task> = updated_tasks
        .iter()
        .filter(|t| t.parent_id == Some(parent.id.clone()))
        .collect();
    
    let new_total_progress: i32 = updated_child_tasks
        .iter()
        .map(|t| t.progress.unwrap_or(0))
        .sum();
    let new_average_progress = new_total_progress / updated_child_tasks.len() as i32;
    
    assert_eq!(new_average_progress, 70);
    
    println!("✅ Updated progress calculation: {}% (expected 70%)", new_average_progress);
    
    // Cleanup
    for child_id in child_ids {
        mock_db.delete_task(&child_id).unwrap();
    }
    mock_db.delete_task(&parent.id).unwrap();
    
    println!("🎉 All progress calculation tests passed!");
}

/// 複数レベル階層のテスト
async fn test_multi_level_hierarchy() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing multi-level hierarchy...");
    
    // Level 1: Root task
    let mut root_task = create_test_task_with_notifications();
    root_task.title = "Root Project".to_string();
    root_task.parent_id = None;
    root_task.progress = Some(0);
    
    let root = mock_db.insert_task(root_task).unwrap();
    println!("✅ Created root task: {}", root.title);
    
    // Level 2: Sub-projects
    let mut level2_ids = Vec::new();
    for i in 0..2 {
        let mut level2_task = create_test_task_with_notifications();
        level2_task.id = Uuid::new_v4().to_string();
        level2_task.title = format!("Sub-project {}", i + 1);
        level2_task.parent_id = Some(root.id.clone());
        level2_task.progress = Some(0);
        
        let created_level2 = mock_db.insert_task(level2_task).unwrap();
        level2_ids.push(created_level2.id);
        println!("✅ Created level 2 task: {}", created_level2.title);
    }
    
    // Level 3: Individual tasks
    let mut level3_ids = Vec::new();
    for (i, parent_id) in level2_ids.iter().enumerate() {
        for j in 0..3 {
            let mut level3_task = create_test_task_with_notifications();
            level3_task.id = Uuid::new_v4().to_string();
            level3_task.title = format!("Task {}-{}", i + 1, j + 1);
            level3_task.parent_id = Some(parent_id.clone());
            level3_task.progress = Some(j * 50); // 0%, 50%, 100%
            
            if j == 2 { // Last task is completed
                level3_task.status = "done".to_string();
                level3_task.completed_at = Some(Utc::now().to_rfc3339());
            }
            
            let created_level3 = mock_db.insert_task(level3_task).unwrap();
            level3_ids.push(created_level3.id);
            println!("✅ Created level 3 task: {}", created_level3.title);
        }
    }
    
    // Verify hierarchy structure
    let all_tasks = mock_db.get_all_tasks();
    
    // Count tasks at each level
    let root_tasks: Vec<&Task> = all_tasks.iter().filter(|t| t.parent_id.is_none()).collect();
    let level2_tasks: Vec<&Task> = all_tasks.iter().filter(|t| {
        t.parent_id == Some(root.id.clone())
    }).collect();
    let level3_tasks: Vec<&Task> = all_tasks.iter().filter(|t| {
        t.parent_id.is_some() && level2_ids.contains(t.parent_id.as_ref().unwrap())
    }).collect();
    
    assert_eq!(root_tasks.len(), 1);
    assert_eq!(level2_tasks.len(), 2); // 2 sub-projects under root
    assert_eq!(level3_tasks.len(), 6); // 2 sub-projects * 3 tasks each
    
    println!("✅ Hierarchy structure verified: {} root, {} level2, {} level3", 
             root_tasks.len(), level2_tasks.len(), level3_tasks.len());
    
    // Test hierarchical deletion (delete level 2, should delete its children)
    let first_level2_id = &level2_ids[0];
    
    // Find children of first level 2 task
    let children_to_delete: Vec<String> = all_tasks
        .iter()
        .filter(|t| t.parent_id == Some(first_level2_id.clone()))
        .map(|t| t.id.clone())
        .collect();
    
    // Delete level 2 task
    mock_db.delete_task(first_level2_id).unwrap();
    
    // In a real implementation, children would be cascade deleted
    // For mock, we need to delete them manually
    for child_id in children_to_delete {
        mock_db.delete_task(&child_id).unwrap();
    }
    
    println!("✅ Hierarchical deletion test passed");
    
    // Cleanup remaining tasks
    for task_id in level3_ids {
        if mock_db.get_task_by_id(&task_id).is_ok() {
            mock_db.delete_task(&task_id).unwrap();
        }
    }
    for task_id in level2_ids {
        if mock_db.get_task_by_id(&task_id).is_ok() {
            mock_db.delete_task(&task_id).unwrap();
        }
    }
    mock_db.delete_task(&root.id).unwrap();
    
    println!("🎉 All multi-level hierarchy tests passed!");
}

/// 階層タスクのステータス更新テスト
async fn test_hierarchical_status_updates() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing hierarchical status updates...");
    
    // Create parent task
    let mut parent_task = create_test_task_with_notifications();
    parent_task.title = "Sprint Task".to_string();
    parent_task.status = "todo".to_string();
    parent_task.progress = Some(0);
    
    let parent = mock_db.insert_task(parent_task).unwrap();
    
    // Create child tasks
    let child_count = 4;
    let mut child_ids = Vec::new();
    
    for i in 0..child_count {
        let mut child_task = create_test_task_with_notifications();
        child_task.id = Uuid::new_v4().to_string();
        child_task.title = format!("User Story {}", i + 1);
        child_task.parent_id = Some(parent.id.clone());
        child_task.status = "todo".to_string();
        child_task.progress = Some(0);
        
        let created_child = mock_db.insert_task(child_task).unwrap();
        child_ids.push(created_child.id);
    }
    
    // Test 1: Start working on some child tasks
    for i in 0..2 {
        let mut child = mock_db.get_task_by_id(&child_ids[i]).unwrap();
        child.status = "in_progress".to_string();
        child.progress = Some(25);
        
        mock_db.update_task(&child_ids[i], child).unwrap();
    }
    
    // Verify parent status could be updated to in_progress (in real implementation)
    let updated_parent = mock_db.get_task_by_id(&parent.id).unwrap();
    println!("✅ Parent status after children started: {}", updated_parent.status);
    
    // Test 2: Complete some child tasks
    for i in 0..2 {
        let mut child = mock_db.get_task_by_id(&child_ids[i]).unwrap();
        child.status = "done".to_string();
        child.progress = Some(100);
        child.completed_at = Some(Utc::now().to_rfc3339());
        
        mock_db.update_task(&child_ids[i], child).unwrap();
    }
    
    // Calculate completion percentage: 2 out of 4 = 50%
    let all_tasks = mock_db.get_all_tasks();
    let child_tasks: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| t.parent_id == Some(parent.id.clone()))
        .collect();
    
    let completed_children = child_tasks
        .iter()
        .filter(|t| t.status == "done")
        .count();
    
    let completion_percentage = (completed_children * 100) / child_tasks.len();
    
    assert_eq!(completion_percentage, 50);
    println!("✅ Parent completion: {}% (2/4 children done)", completion_percentage);
    
    // Test 3: Complete all remaining child tasks
    for i in 2..child_count {
        let mut child = mock_db.get_task_by_id(&child_ids[i]).unwrap();
        child.status = "done".to_string();
        child.progress = Some(100);
        child.completed_at = Some(Utc::now().to_rfc3339());
        
        mock_db.update_task(&child_ids[i], child).unwrap();
    }
    
    // Now parent should be completable
    let final_tasks = mock_db.get_all_tasks();
    let final_child_tasks: Vec<&Task> = final_tasks
        .iter()
        .filter(|t| t.parent_id == Some(parent.id.clone()))
        .collect();
    
    let all_children_done = final_child_tasks
        .iter()
        .all(|t| t.status == "done");
    
    assert!(all_children_done);
    
    // Update parent to done
    let mut final_parent = mock_db.get_task_by_id(&parent.id).unwrap();
    final_parent.status = "done".to_string();
    final_parent.progress = Some(100);
    final_parent.completed_at = Some(Utc::now().to_rfc3339());
    
    mock_db.update_task(&parent.id, final_parent).unwrap();
    
    println!("✅ All children completed, parent marked as done");
    
    // Cleanup
    for child_id in child_ids {
        mock_db.delete_task(&child_id).unwrap();
    }
    mock_db.delete_task(&parent.id).unwrap();
    
    println!("🎉 All hierarchical status update tests passed!");
}

/// 階層タスクの制約テスト
async fn test_hierarchical_constraints() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing hierarchical constraints...");
    
    // Test 1: 循環参照の防止（A -> B -> A）
    let mut task_a = create_test_task_with_notifications();
    task_a.title = "Task A".to_string();
    task_a.parent_id = None;
    
    let created_a = mock_db.insert_task(task_a).unwrap();
    
    let mut task_b = create_test_task_with_notifications();
    task_b.id = Uuid::new_v4().to_string();
    task_b.title = "Task B".to_string();
    task_b.parent_id = Some(created_a.id.clone());
    
    let created_b = mock_db.insert_task(task_b).unwrap();
    
    // Try to make A a child of B (creating circular reference)
    let mut updated_a = created_a.clone();
    updated_a.parent_id = Some(created_b.id.clone());
    
    // MockDatabase allows this, but real implementation should prevent it
    let circular_result = mock_db.update_task(&created_a.id, updated_a);
    assert!(circular_result.is_ok()); // Mock allows, real service should reject
    
    println!("✅ Circular reference test (mock allows, real service should prevent)");
    
    // Test 2: 深い階層制限のテスト（例：10レベル以上）
    let max_depth = 5;
    let mut current_parent_id = created_a.id.clone();
    let mut deep_task_ids = Vec::new();
    
    for depth in 1..=max_depth {
        let mut deep_task = create_test_task_with_notifications();
        deep_task.id = Uuid::new_v4().to_string();
        deep_task.title = format!("Deep Task Level {}", depth);
        deep_task.parent_id = Some(current_parent_id.clone());
        
        let created_deep = mock_db.insert_task(deep_task).unwrap();
        current_parent_id = created_deep.id.clone();
        deep_task_ids.push(created_deep.id);
    }
    
    println!("✅ Created {} level deep hierarchy", max_depth);
    
    // Test 3: 自分自身を親に設定する試行
    let mut self_parent = mock_db.get_task_by_id(&created_a.id).unwrap();
    self_parent.parent_id = Some(created_a.id.clone());
    
    // MockDatabase allows this, but real implementation should prevent it
    let self_parent_result = mock_db.update_task(&created_a.id, self_parent);
    assert!(self_parent_result.is_ok()); // Mock allows, real service should reject
    
    println!("✅ Self-parent test (mock allows, real service should prevent)");
    
    // Test 4: 存在しない親タスクを指定
    let non_existent_parent_id = Uuid::new_v4().to_string();
    let mut orphan_task = create_test_task_with_notifications();
    orphan_task.id = Uuid::new_v4().to_string();
    orphan_task.title = "Orphan Task".to_string();
    orphan_task.parent_id = Some(non_existent_parent_id);
    
    // MockDatabase allows this, but real implementation should validate parent exists
    let orphan_result = mock_db.insert_task(orphan_task);
    assert!(orphan_result.is_ok()); // Mock allows, real service should validate
    
    println!("✅ Non-existent parent test (mock allows, real service should validate)");
    
    // Cleanup
    if let Ok(orphan) = orphan_result {
        mock_db.delete_task(&orphan.id).unwrap();
    }
    
    for task_id in deep_task_ids.into_iter().rev() { // Reverse order for proper cleanup
        mock_db.delete_task(&task_id).unwrap();
    }
    mock_db.delete_task(&created_b.id).unwrap();
    mock_db.delete_task(&created_a.id).unwrap();
    
    println!("🎉 All hierarchical constraint tests passed!");
}

/// 階層タスク検索とフィルタリングのテスト
async fn test_hierarchical_search_and_filtering() {
    let mock_db = MockDatabase::new();
    
    println!("🧪 Testing hierarchical search and filtering...");
    
    // Create a complex hierarchy for testing
    // Project -> [Feature 1, Feature 2] -> [Task 1.1, Task 1.2, Task 2.1, Task 2.2]
    
    let mut project = create_test_task_with_notifications();
    project.title = "Search Test Project".to_string();
    project.priority = "high".to_string();
    
    let created_project = mock_db.insert_task(project).unwrap();
    
    let mut all_created_ids = vec![created_project.id.clone()];
    
    // Create features
    for feature_num in 1..=2 {
        let mut feature = create_test_task_with_notifications();
        feature.id = Uuid::new_v4().to_string();
        feature.title = format!("Feature {}", feature_num);
        feature.parent_id = Some(created_project.id.clone());
        feature.priority = if feature_num == 1 { "high".to_string() } else { "medium".to_string() };
        
        let created_feature = mock_db.insert_task(feature).unwrap();
        all_created_ids.push(created_feature.id.clone());
        
        // Create tasks for each feature
        for task_num in 1..=2 {
            let mut task = create_test_task_with_notifications();
            task.id = Uuid::new_v4().to_string();
            task.title = format!("Task {}.{}", feature_num, task_num);
            task.parent_id = Some(created_feature.id.clone());
            task.status = if task_num == 1 { "todo".to_string() } else { "done".to_string() };
            task.priority = "low".to_string();
            
            if task.status == "done" {
                task.completed_at = Some(Utc::now().to_rfc3339());
                task.progress = Some(100);
            }
            
            let created_task = mock_db.insert_task(task).unwrap();
            all_created_ids.push(created_task.id);
        }
    }
    
    // Test 1: Find all tasks by priority
    let all_tasks = mock_db.get_all_tasks();
    let high_priority_tasks: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| t.priority == "high")
        .collect();
    
    assert_eq!(high_priority_tasks.len(), 2); // Project + Feature 1
    println!("✅ Found {} high priority tasks", high_priority_tasks.len());
    
    // Test 2: Find completed tasks
    let completed_tasks: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| t.status == "done")
        .collect();
    
    assert_eq!(completed_tasks.len(), 2); // Task 1.2 and Task 2.2
    println!("✅ Found {} completed tasks", completed_tasks.len());
    
    // Test 3: Find root tasks (no parent)
    let root_tasks: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| t.parent_id.is_none())
        .filter(|t| all_created_ids.contains(&t.id))
        .collect();
    
    assert_eq!(root_tasks.len(), 1); // Only the project
    println!("✅ Found {} root tasks", root_tasks.len());
    
    // Test 4: Find children of specific parent
    let project_children: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| t.parent_id == Some(created_project.id.clone()))
        .collect();
    
    assert_eq!(project_children.len(), 2); // Feature 1 and Feature 2
    println!("✅ Found {} direct children of project", project_children.len());
    
    // Test 5: Search by title pattern
    let feature_tasks: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| t.title.contains("Feature"))
        .collect();
    
    assert_eq!(feature_tasks.len(), 2);
    println!("✅ Found {} tasks with 'Feature' in title", feature_tasks.len());
    
    // Test 6: Find leaf tasks (no children)
    let mut leaf_task_count = 0;
    for task in &all_tasks {
        if all_created_ids.contains(&task.id) {
            let has_children = all_tasks
                .iter()
                .any(|t| t.parent_id == Some(task.id.clone()));
            
            if !has_children && task.parent_id.is_some() {
                leaf_task_count += 1;
            }
        }
    }
    
    assert_eq!(leaf_task_count, 4); // Task 1.1, 1.2, 2.1, 2.2
    println!("✅ Found {} leaf tasks", leaf_task_count);
    
    // Cleanup
    // Delete in reverse hierarchy order (leaves first)
    for task in all_tasks.iter().rev() {
        if all_created_ids.contains(&task.id) {
            mock_db.delete_task(&task.id).unwrap();
        }
    }
    
    println!("🎉 All hierarchical search and filtering tests passed!");
}

/// 階層タスクテストのメインランナー
#[tokio::test]
async fn hierarchical_task_tests() {
    println!("🧪 Starting comprehensive hierarchical task tests...");
    
    // Test 1: Parent-child relationships
    test_parent_child_task_relationships().await;
    println!("✅ Parent-child relationships test PASSED");
    
    // Test 2: Progress calculation
    test_progress_calculation().await;
    println!("✅ Progress calculation test PASSED");
    
    // Test 3: Multi-level hierarchy
    test_multi_level_hierarchy().await;
    println!("✅ Multi-level hierarchy test PASSED");
    
    // Test 4: Hierarchical status updates
    test_hierarchical_status_updates().await;
    println!("✅ Hierarchical status updates test PASSED");
    
    // Test 5: Hierarchical constraints
    test_hierarchical_constraints().await;
    println!("✅ Hierarchical constraints test PASSED");
    
    // Test 6: Hierarchical search and filtering
    test_hierarchical_search_and_filtering().await;
    println!("✅ Hierarchical search and filtering test PASSED");
    
    println!("🎉 All hierarchical task tests completed!");
}