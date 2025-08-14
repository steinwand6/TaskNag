// 通知設定テストを直接実行するスクリプト
// Node.js環境で実行される想定

const { testNotificationSettingsIntegration } = require('./src/tests/notificationSettings.test.ts');

async function runTest() {
    console.log('🚀 通知設定テストを開始します...');
    
    try {
        const success = await testNotificationSettingsIntegration();
        
        if (success) {
            console.log('🎉 すべてのテストが成功しました！');
            process.exit(0);
        } else {
            console.log('❌ テストが失敗しました');
            process.exit(1);
        }
    } catch (error) {
        console.error('❌ テスト実行中にエラーが発生しました:', error);
        process.exit(1);
    }
}

runTest();