"use client"
import { redirect } from 'next/navigation';
import {useState } from 'react';
import MainView from './main';


function LoginErrorView({err_msg} : { err_msg: string }) {
  if (err_msg) {
    return <span>{err_msg}</span>
  } else {
    <></>
  }
}

export default function Home() {
  const [login_cookie, setLoginCookie] = useState<string>('');
  const [username, setUsername] = useState<string>('');
  const [password, setPassword] = useState<string>('');
  const [msg, setMsg] = useState<string>('');

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
        redirect('/main');
      })
      .catch((e) => setMsg("Error: " + e));
  }

  if (!login_cookie)
    return (
       <div className="grid grid-rows-[20px_1fr_20px] items-cente
  r justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        <LoginErrorView err_msg = {msg}/>
      <form onSubmit={login} >
        <input type="text" placeholder="Name" className="input input-bordered w-full max-w-xs" value={username} onChange={(e) => setUsername(e.target.value)}/>
        <input type="password" placeholder="Password" className="input input-bordered w-full max-w-xs" value={password} onChange={(e) => setPassword(e.target.value)}/>
        <button className="btn" type="submit">Login</button>
      </form>
      </main>
    </div>
  );
  else {
    return (
      <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
        <MainView login_cookie={login_cookie} />
        </div>
    );
  }
}
