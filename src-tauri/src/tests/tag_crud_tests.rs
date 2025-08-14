use crate::models::CreateTagRequest;
use crate::error::AppError;
use crate::services::TagService;
use sqlx::{Pool, Sqlite, SqlitePool};

// テスト用のインメモリデータベース接続を作成
async fn create_test_pool() -> Pool<Sqlite> {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    
    // マイグレーションを実行
    crate::database::migrations::run_migrations(&pool).await.unwrap();
    
    pool
}

/// タグ基本CRUD操作のテスト
#[tokio::test]
async fn test_basic_tag_crud_operations() {
    let pool = create_test_pool().await;
    
    println!("🧪 Testing basic tag CRUD operations...");
    
    // Test 1: タグ作成
    let create_request = CreateTagRequest {
        name: "仕事".to_string(),
        color: "#FF5733".to_string(),
    };
    
    let created_tag = TagService::create_tag(&pool, create_request).await.unwrap();
    
    assert_eq!(created_tag.name, "仕事");
    assert_eq!(created_tag.color, "#FF5733");
    assert!(!created_tag.id.is_empty());
    
    println!("✅ Tag creation test passed");
    
    // Test 2: タグ取得
    let retrieved_tag = TagService::get_tag_by_id(&pool, &created_tag.id).await.unwrap();
    
    assert_eq!(retrieved_tag.id, created_tag.id);
    assert_eq!(retrieved_tag.name, created_tag.name);
    
    println!("✅ Tag retrieval test passed");
    
    // Test 3: 全タグ取得
    let all_tags = TagService::get_all_tags(&pool).await.unwrap();
    assert!(!all_tags.is_empty());
    
    println!("✅ Get all tags test passed");
    
    // Test 4: タグ削除
    TagService::delete_tag(&pool, &created_tag.id).await.unwrap();
    
    // 削除確認
    let delete_result = TagService::get_tag_by_id(&pool, &created_tag.id).await;
    assert!(delete_result.is_err());
    
    println!("✅ Tag deletion test passed");
    
    println!("🎉 All basic tag CRUD tests passed!");
}

/// タグ名重複検証のテスト
#[tokio::test]
async fn test_tag_name_duplication_validation() {
    let pool = create_test_pool().await;
    
    println!("🧪 Testing tag name duplication validation...");
    
    // Test 1: 最初のタグ作成
    let create_request = CreateTagRequest {
        name: "重複テスト".to_string(),
        color: "#FF5733".to_string(),
    };
    
    let first_tag = TagService::create_tag(&pool, create_request).await.unwrap();
    assert_eq!(first_tag.name, "重複テスト");
    
    // Test 2: 同じ名前でタグ作成（重複エラー）
    let duplicate_request = CreateTagRequest {
        name: "重複テスト".to_string(),
        color: "#33FF57".to_string(),
    };
    
    let duplicate_result = TagService::create_tag(&pool, duplicate_request).await;
    assert!(duplicate_result.is_err());
    match duplicate_result {
        Err(AppError::Validation(_)) => println!("✅ Duplicate name validation correctly triggered"),
        _ => panic!("Expected Validation error for duplicate name"),
    }
    
    // Cleanup
    TagService::delete_tag(&pool, &first_tag.id).await.unwrap();
    
    println!("🎉 All tag name duplication validation tests passed!");
}

/// エラーケースのテスト
#[tokio::test]
async fn test_tag_error_cases() {
    let pool = create_test_pool().await;
    
    println!("🧪 Testing tag error cases...");
    
    // Test 1: 存在しないタグの取得
    let non_existent_id = uuid::Uuid::new_v4().to_string();
    let result = TagService::get_tag_by_id(&pool, &non_existent_id).await;
    
    assert!(result.is_err());
    match result {
        Err(AppError::NotFound(_)) => println!("✅ NotFound error correctly returned for non-existent tag"),
        _ => panic!("Expected NotFound error"),
    }
    
    // Test 2: 存在しないタグの削除
    let delete_result = TagService::delete_tag(&pool, &non_existent_id).await;
    assert!(delete_result.is_err());
    match delete_result {
        Err(AppError::NotFound(_)) => println!("✅ Delete NotFound error correctly returned"),
        _ => panic!("Expected NotFound error for delete"),
    }
    
    println!("🎉 All tag error case tests passed!");
}