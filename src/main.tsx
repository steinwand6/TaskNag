import React from 'react'
import ReactDOM from 'react-dom/client'
import { MemoryRouter, Routes, Route } from 'react-router-dom'
import App from './App.tsx'
import { TaskDetailPage } from './pages/TaskDetailPage.tsx'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <MemoryRouter>
      <Routes>
        <Route path="/" element={<App />} />
        <Route path="/task/:taskId" element={<TaskDetailPage />} />
      </Routes>
    </MemoryRouter>
  </React.StrictMode>,
)