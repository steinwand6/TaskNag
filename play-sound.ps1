# Claude Code タスク完了音再生スクリプト

# タスク完了を明確に示すベル風サウンド

# オプション1: Windows標準のベルサウンド（chimes.wav）
$chimesPath = "C:\Windows\Media\chimes.wav"
if ($chimesPath -and (Test-Path $chimesPath)) {
    $sound = New-Object System.Media.SoundPlayer
    $sound.SoundLocation = $chimesPath
    $sound.PlaySync()
    Write-Host "🔔 タスク完了！ (chimes.wav)"
} else {
    # オプション2: カスタムベルビープ音（高→低の2音）
    [console]::beep(1200, 200)  # 高音 1200Hz, 200ms
    Start-Sleep -Milliseconds 50
    [console]::beep(800, 300)   # 低音 800Hz, 300ms
    Write-Host "🔔 タスク完了！ (ベルビープ)"
}

# 他のベル系音のオプション（コメントを外して使用）

# オプション3: 三重ベル（チャイム風）
# [console]::beep(1000, 150)
# Start-Sleep -Milliseconds 50
# [console]::beep(1200, 150) 
# Start-Sleep -Milliseconds 50
# [console]::beep(1500, 200)

# オプション4: 成功音風（上昇音階）
# [console]::beep(523, 150)   # C5
# Start-Sleep -Milliseconds 25
# [console]::beep(659, 150)   # E5
# Start-Sleep -Milliseconds 25
# [console]::beep(784, 200)   # G5

# オプション5: 他のWindows標準音
# [System.Media.SystemSounds]::Question.Play()  # 質問音
# [System.Media.SystemSounds]::Exclamation.Play()  # 感嘆音