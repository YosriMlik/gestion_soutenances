import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { message } from '@tauri-apps/plugin-dialog';
import { useNavigate } from 'react-router-dom';
import "./Login.css";

function Login() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const navigate = useNavigate();

  useEffect(() => {
    if (localStorage.getItem('token')) {
      navigate('/dashboard/department/8/defences');
    } else {
    }
  }, [navigate]);

  const handleLogin = async () => {
    try {
      // Invoke the Tauri 'login' command
      const response = await invoke('login', { email, password });
      
      // Check if the response contains an access_token
      if (response.access_token) {
        localStorage.setItem('token', response.access_token);
        //await message('User connected', { title: 'Message', kind: 'success' });
        console.log('Token:', response.access_token);
        navigate('/dashboard/department/8/defences');
      } else if(response.message == "Invalid credentials") {
        await message('Invalid credentials', { title: 'Message', kind: 'warning' });;
      }
    } catch (error) {
      console.error('Erreur lors de la connexion:', error);
      await message("An Error Occured : "+error, { title: 'Message', kind: 'error' });
    }
  };

  const handleKeyPress = (e) => {
    if (e.key === 'Enter') {
      handleLogin();
    }
  };

  return (
    <div className="shadow-lg forme-authentification mx-auto">
      <h1 className="text-center mb-5" style={{ fontSize: "2.7rem" }}>
        Se connecter
      </h1>
      <input
        type="email"
        placeholder="Email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        onKeyPress={handleKeyPress}
        className="form-control"
      />
      <br />
      <input
        type="password"
        placeholder="Mot de passe"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        onKeyPress={handleKeyPress}
        className="form-control"
      />
      <br />
      <button
        className="w-100 btn btn-primary rounded-pill mb-3"
        onClick={handleLogin}
      >
        Connecter
      </button>
      <a href="#">récupérer le mot de passe</a>
      <br />
      <br />
    </div>
  );
}

export default Login;