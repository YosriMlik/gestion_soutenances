import React, { useState } from 'react';
import { Link, Outlet, useNavigate } from 'react-router-dom';
import { confirm } from '@tauri-apps/plugin-dialog';

const Dashboard = () => {
  const [isSidebarOpen, setIsSidebarOpen] = useState(false);
  const navigate = useNavigate();

  const openNav = () => {
    setIsSidebarOpen(true);
  };

  const closeNav = () => {
    setIsSidebarOpen(false);
  };

  const logOut = async () => {
    const confirmation = await confirm('Are you sure you want to log out?', {
      title: 'Logout',
      type: 'warning',
    });
    if (confirmation) {
      try {
        // Optionally call a Tauri logout command (uncomment if implemented in Rust)
        // await invoke('logout', { token: localStorage.getItem('token') });
        
        // For now, just clear the token and redirect
        localStorage.removeItem('token');
        navigate("/login");
      } catch (error) {
        console.error('Logout failed:', error);
      }
    }
  };

  return (
    <div>
      {/* Sidebar */}
      <div id="mySidenav" style={isSidebarOpen ? { width: '250px' } : { width: '0px' }} className="sidenav">
        <button className="closebtn" onClick={closeNav}>
          ×
        </button>
        <Link className="_nav2" to="/dashboard/department/4/defences" onClick={closeNav}>
          G.Civil
        </Link>
        <Link className="_nav2" to="/dashboard/department/5/defences" onClick={closeNav}>
          G.Procédés
        </Link>
        <Link className="_nav2" to="/dashboard/department/6/defences" onClick={closeNav}>
          G.Telecom
        </Link>
        <Link className="_nav2" to="/dashboard/department/7/defences" onClick={closeNav}>
          G.indus
        </Link>
        <Link className="_nav2" to="/dashboard/department/8/defences" onClick={closeNav}>
          G.Info
        </Link>
        <Link className="_nav2" to="/dashboard/department/9/defences" onClick={closeNav}>
          G.Méca
        </Link>
        <div className="dropdown">
          <Link className="_nav2" to="#">
            Licences
          </Link>
          <div className="dropdown-content">
            <Link to="/dashboard/department/1/defences" onClick={closeNav}>Génie Industriel</Link>
            <Link to="/dashboard/department/2/defences" onClick={closeNav}>Informatique</Link>
          </div>
        </div>
        <div className="dropdown">
          <Link className="_nav2" to="#">
            Masères
          </Link>
          <div className="dropdown-content">
            <Link to="/dashboard/department/3/defences" onClick={closeNav}>Industrie V4.0</Link>
          </div>
        </div>
      </div>

      {/* Navigation Bar */}
      <nav className="row g-0 w-100 nave sticky-top shadow-sm">
        <div className="col-auto">
          <button
            className="btn btn-danger rounded-0"
            style={{ fontSize: '1.1rem' }}
            onClick={(e) => {
              e.preventDefault();
              logOut();
            }}
          >
            Déconnexion <i className="bi bi-box-arrow-left"></i>
          </button>
          <div className="text-end dropdown" style={{ display: 'inline' }}>
            <Link className="btn btn-primary ms-2 px-3 rounded-0" to="#Gérer">
              <i className="bi bi-sliders2-vertical"></i>
              <span className="ms-1" style={{ fontSize: '1.1rem' }}>Gérer</span>
            </Link>
            <div className="dropdown-content" style={{ left: '0.5rem', fontSize: '1.1rem' }}>
              <Link to="/dashboard/juries">Jurys</Link>
              <Link to="/dashboard/invitees">Invités</Link>
              <Link to="/dashboard/classrooms">Salles</Link>
            </div>
          </div>
        </div>

        <div className="col nav-items text-end">
          <div className="dropdown">
            <Link className="_link" to="#">
              Licences
            </Link>
            <div className="dropdown-content">
              <Link to="/dashboard/department/1/defences">Génie Industriel</Link>
              <Link to="/dashboard/department/2/defences">Informatique</Link>
            </div>
          </div>

          <div className="dropdown">
            <Link className="_link" to="#">
              Masères
            </Link>
            <div className="dropdown-content">
              <Link to="/dashboard/department/3/defences">Industrie V4.0</Link>
            </div>
          </div>
          <Link className="_link" to="/dashboard/department/4/defences">
            G.Civil
          </Link>
          <Link className="_link" to="/dashboard/department/5/defences">
            G.Procédés
          </Link>
          <Link className="_link" to="/dashboard/department/6/defences">
            G.Telecom
          </Link>
          <Link className="_link" to="/dashboard/department/7/defences">
            G.indus
          </Link>
          <Link className="_link" to="/dashboard/department/8/defences">
            G.Info
          </Link>
          <Link className="_link" to="/dashboard/department/9/defences">
            G.Méca
          </Link>
        </div>
        <a className="col-auto ms-auto text-end sidebar-icon" onClick={openNav}>
          ☰
        </a>
      </nav>
      {/* Main Content */}
      <div className='_container mt-5'>
        <Outlet />
      </div>
    </div>
  );
};

export default Dashboard;