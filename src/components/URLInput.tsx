import React, { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { 
  URLInputProps, 
  URLValidationResult, 
  URLPreview,
  URL_PATTERNS 
} from '../types/BrowserAction';

const URLInput: React.FC<URLInputProps> = ({
  value,
  onChange,
  onValidation,
  placeholder = "https://example.com",
  disabled = false,
  showPreview = true,
  showTestButton = true,
  error: externalError
}) => {
  const [validation, setValidation] = useState<URLValidationResult>({ isValid: true });
  const [preview, setPreview] = useState<URLPreview | null>(null);
  const [isValidating, setIsValidating] = useState(false);
  const [isTesting, setIsTesting] = useState(false);
  const [isLoadingPreview, setIsLoadingPreview] = useState(false);

  // Basic client-side validation
  const validateURL = useCallback((url: string): URLValidationResult => {
    if (!url.trim()) {
      return { isValid: true }; // Empty is OK
    }

    // Check for dangerous protocols
    const lowerUrl = url.toLowerCase();
    for (const protocol of URL_PATTERNS.DANGEROUS_PROTOCOLS) {
      if (lowerUrl.startsWith(protocol)) {
        return {
          isValid: false,
          error: `危険なプロトコル "${protocol}" は使用できません`,
          suggestions: [`https://${url.replace(/^[^:]+:\/?\/?/, '')}`]
        };
      }
    }

    // Check if URL is too long
    if (url.length > 2000) {
      return {
        isValid: false,
        error: 'URLが長すぎます（2000文字以内）'
      };
    }

    // Basic URL format check
    try {
      new URL(url);
      return { isValid: true };
    } catch {
      // Try to suggest corrections
      const suggestions: string[] = [];
      
      if (!url.includes('://')) {
        suggestions.push(`https://${url}`);
        if (!url.startsWith('www.')) {
          suggestions.push(`https://www.${url}`);
        }
      }
      
      return {
        isValid: false,
        error: '有効なURLを入力してください',
        suggestions
      };
    }
  }, []);

  // Server-side validation
  const validateURLWithServer = useCallback(async (url: string) => {
    if (!url.trim()) return;
    
    setIsValidating(true);
    try {
      const result = await invoke<URLValidationResult>('validate_url_command', { url });
      setValidation(result);
      onValidation?.(result);
    } catch (error) {
      const errorResult = {
        isValid: false,
        error: `バリデーションエラー: ${error}`
      };
      setValidation(errorResult);
      onValidation?.(errorResult);
    } finally {
      setIsValidating(false);
    }
  }, [onValidation]);

  // Get URL preview
  const getURLPreview = useCallback(async (url: string) => {
    if (!url.trim() || !validation.isValid) return;
    
    setIsLoadingPreview(true);
    try {
      const previewData = await invoke<URLPreview>('get_url_preview_command', { url });
      setPreview(previewData);
    } catch (error) {
      console.warn('Failed to get URL preview:', error);
      setPreview(null);
    } finally {
      setIsLoadingPreview(false);
    }
  }, [validation.isValid]);

  // Test URL by opening it
  const testURL = useCallback(async () => {
    if (!value.trim() || !validation.isValid) return;
    
    setIsTesting(true);
    try {
      await invoke('test_url_command', { url: value });
      // Show success feedback
      const successDiv = document.createElement('div');
      successDiv.className = 'fixed top-4 right-4 bg-green-500 text-white px-4 py-2 rounded shadow z-50';
      successDiv.textContent = 'URLを開きました';
      document.body.appendChild(successDiv);
      setTimeout(() => successDiv.remove(), 3000);
    } catch (error) {
      console.error('Failed to test URL:', error);
      // Show error feedback
      const errorDiv = document.createElement('div');
      errorDiv.className = 'fixed top-4 right-4 bg-red-500 text-white px-4 py-2 rounded shadow z-50';
      errorDiv.textContent = `URL テスト失敗: ${error}`;
      document.body.appendChild(errorDiv);
      setTimeout(() => errorDiv.remove(), 5000);
    } finally {
      setIsTesting(false);
    }
  }, [value, validation.isValid]);

  // Handle input change
  const handleChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = e.target.value;
    onChange(newValue);
    
    // Immediate client-side validation
    const clientValidation = validateURL(newValue);
    setValidation(clientValidation);
    onValidation?.(clientValidation);
    
    // Clear preview when URL changes
    setPreview(null);
  }, [onChange, validateURL, onValidation]);

  // Debounced server validation
  useEffect(() => {
    const timer = setTimeout(() => {
      if (value && validation.isValid) {
        validateURLWithServer(value);
      }
    }, 500);

    return () => clearTimeout(timer);
  }, [value, validation.isValid, validateURLWithServer]);

  // Get preview when validation succeeds
  useEffect(() => {
    if (showPreview && value && validation.isValid && !isValidating) {
      const timer = setTimeout(() => {
        getURLPreview(value);
      }, 1000);
      
      return () => clearTimeout(timer);
    }
  }, [value, validation.isValid, isValidating, showPreview, getURLPreview]);

  // Apply suggestion
  const applySuggestion = useCallback((suggestion: string) => {
    onChange(suggestion);
    const suggestionValidation = validateURL(suggestion);
    setValidation(suggestionValidation);
    onValidation?.(suggestionValidation);
  }, [onChange, validateURL, onValidation]);

  const hasError = externalError || (!validation.isValid && value.trim());
  const errorMessage = externalError || validation.error;

  return (
    <div className="space-y-2">
      {/* Input field */}
      <div className="relative">
        <input
          type="url"
          value={value}
          onChange={handleChange}
          placeholder={placeholder}
          disabled={disabled}
          className={`
            w-full px-3 py-2 border rounded-md transition-colors
            ${hasError 
              ? 'border-red-500 focus:border-red-500 focus:ring-red-200' 
              : 'border-gray-300 focus:border-blue-500 focus:ring-blue-200'
            }
            ${disabled ? 'bg-gray-100 cursor-not-allowed' : 'bg-white'}
            focus:outline-none focus:ring-2
          `}
        />
        
        {/* Validation indicator */}
        <div className="absolute right-3 top-1/2 transform -translate-y-1/2 flex items-center space-x-2">
          {isValidating && (
            <div className="w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full animate-spin" />
          )}
          
          {!isValidating && value.trim() && (
            <div className={`w-2 h-2 rounded-full ${validation.isValid ? 'bg-green-500' : 'bg-red-500'}`} />
          )}
          
          {showTestButton && validation.isValid && value.trim() && (
            <button
              type="button"
              onClick={testURL}
              disabled={isTesting || disabled}
              className={`
                px-2 py-1 text-xs rounded border transition-colors
                ${isTesting || disabled
                  ? 'bg-gray-100 text-gray-400 cursor-not-allowed'
                  : 'bg-blue-50 text-blue-600 border-blue-200 hover:bg-blue-100'
                }
              `}
            >
              {isTesting ? 'テスト中...' : 'テスト'}
            </button>
          )}
        </div>
      </div>

      {/* Error message */}
      {hasError && (
        <div className="text-red-600 text-sm flex items-start space-x-2">
          <svg className="w-4 h-4 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
            <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
          </svg>
          <span>{errorMessage}</span>
        </div>
      )}

      {/* Suggestions */}
      {validation.suggestions && validation.suggestions.length > 0 && (
        <div className="space-y-1">
          <p className="text-sm text-gray-600">候補:</p>
          <div className="flex flex-wrap gap-2">
            {validation.suggestions.map((suggestion, index) => (
              <button
                key={index}
                type="button"
                onClick={() => applySuggestion(suggestion)}
                disabled={disabled}
                className="px-2 py-1 text-xs bg-gray-100 text-gray-700 rounded hover:bg-gray-200 transition-colors disabled:cursor-not-allowed"
              >
                {suggestion}
              </button>
            ))}
          </div>
        </div>
      )}

      {/* URL Preview */}
      {showPreview && preview && (
        <div className="p-3 bg-gray-50 rounded-md border">
          <div className="flex items-start space-x-3">
            {/* Favicon placeholder */}
            <div className="w-6 h-6 bg-blue-100 rounded flex items-center justify-center flex-shrink-0">
              <svg className="w-4 h-4 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                <path fillRule="evenodd" d="M4.083 9h1.946c.089-1.546.383-2.97.837-4.118A6.004 6.004 0 004.083 9zM10 2a8 8 0 100 16 8 8 0 000-16zm0 2c-.076 0-.232.032-.465.262-.238.234-.497.623-.737 1.182-.389.907-.673 2.142-.766 3.556h3.936c-.093-1.414-.377-2.649-.766-3.556-.24-.56-.5-.948-.737-1.182C10.232 4.032 10.076 4 10 4zm3.971 5c-.089-1.546-.383-2.97-.837-4.118A6.004 6.004 0 0115.917 9h-1.946zm-2.003 2H8.032c.093 1.414.377 2.649.766 3.556.24.56.5.948.737 1.182.233.23.389.262.465.262.076 0 .232-.032.465-.262.238-.234.498-.623.737-1.182.389-.907.673-2.142.766-3.556zm1.166 4.118c.454-1.147.748-2.572.837-4.118h1.946a6.004 6.004 0 01-2.783 4.118zm-6.268 0C6.412 13.97 6.118 12.546 6.03 11H4.083a6.004 6.004 0 002.783 4.118z" clipRule="evenodd" />
              </svg>
            </div>
            
            {/* Preview content */}
            <div className="flex-1 min-w-0">
              <h4 className="font-medium text-gray-900 truncate">{preview.title}</h4>
              <p className="text-sm text-gray-600 truncate">{preview.domain}</p>
              {preview.description && (
                <p className="text-sm text-gray-500 mt-1 line-clamp-2">{preview.description}</p>
              )}
            </div>
          </div>
        </div>
      )}

      {/* Loading preview indicator */}
      {isLoadingPreview && (
        <div className="flex items-center space-x-2 text-sm text-gray-500">
          <div className="w-4 h-4 border-2 border-gray-300 border-t-gray-600 rounded-full animate-spin" />
          <span>プレビューを読み込み中...</span>
        </div>
      )}
    </div>
  );
};

export default URLInput;