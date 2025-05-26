import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import Login from './pages/Login';
import Dashboard from './pages/Dashboard';
import PrivateRoute from './PrivateRoute';
import Defences from './components/Defences';
import Invitees from './components/Invitees';
import Juries from './components/Juries';
import Classrooms from './components/Classrooms';
import Students from './components/Students';

import './assets/css/style.css';

createRoot(document.getElementById('root')).render(                     
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/login" element={<Login />} />

        <Route element={<PrivateRoute />}>
          <Route path="/dashboard/*" element={<Dashboard />}>
              {/* Nested route for SpecialiteDetails */}
              <Route path="department/:id/defences" element={<Defences />} />
              <Route path="department/:id/students" element={<Students />} />
              <Route path="invitees" element={<Invitees />} />
              <Route path="juries" element={<Juries />} />
              <Route path="classrooms" element={<Classrooms />} />
          </Route>
        </Route>

        <Route path="*" element={<Navigate to="/login" />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
)