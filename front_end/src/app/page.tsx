"use client"
import { redirect } from 'next/navigation';
import { useState } from 'react';
import MainView from './main';
import Link from 'next/link';


function LoginErrorView({ err_msg }: { err_msg: string }) {
  if (err_msg) {
    return <span>{err_msg}</span>
  } else {
    <></>
  }
}

export default function Home() {
  const [user_id, setUserId] = useState<number>(-1);
  const [username, setUsername] = useState<string>('');
  const [password, setPassword] = useState<string>('');
  const [msg, setMsg] = useState<string>('');

  const login = (e: Event) => {
    e.preventDefault();
    fetch('/auth/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username: username,
        password: password,
      })
    })
      .then((res) => res.json())
      .then((result) => {
        const ses = fetch('/auth/session');
        console.log(ses);
        setUserId(result.user_id);
        setUsername('');
        setPassword('');
      })
      .catch((e) => setMsg("Error: " + e));
  }

  if (user_id === -1)
    return (
      <div className="grid grid-rows-[20px_1fr_20px] items-cente
  r justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
        <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
          <LoginErrorView err_msg={msg} />
          <form onSubmit={login} >
            <input type="text" placeholder="Name" className="input input-bordered w-full max-w-xs" value={username} onChange={(e) => setUsername(e.target.value)} />
            <input type="password" placeholder="Password" className="input input-bordered w-full max-w-xs" value={password} onChange={(e) => setPassword(e.target.value)} />
            <button className="btn" type="submit">Login</button>
          </form>
          <div className="grid grid-rows">
            <Link href={"/createuser"}>
              <button className="btn">Create User</button>
            </Link>
          </div>
        </main >
      </div >
    );
  else {
    return (
      <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
        <MainView user_id={user_id} />
      </div>
    );
  }
}
