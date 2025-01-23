"use client"
import {useState } from 'react';

export default function Home() {
  const [login_cookie, setLoginCookie] = useState('');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  const login = () => {
    fetch('/api/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        name: username,
        password: password,
      })
    })
      .then((res) => res.json())
      .then((result) => {
        setLoginCookie(result.cookie);
        setUsername('');
        setPassword('');
      })
      .catch((err) => console.log('error'))
  }


  if (!login_cookie)
    return (
      <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        <form onSubmit={login} >
          <input type="text" placeholder="Name" className="input input-bordered w-full max-w-xs" value={username} onChange={(e) => setUsername(e.target.value)}/>
          <input type="password" placeholder="Password" className="input input-bordered w-full max-w-xs" value={password} onChange={(e) => setPassword(e.target.value)}/>
          <button type="submit">Login</button>
        </form>
      </main>
    </div>
  );
  else {
    return (
      <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
        NOTHING
        </div>
    );
  }
}
