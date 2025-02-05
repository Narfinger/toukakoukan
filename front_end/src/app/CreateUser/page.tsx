"use client"

import { redirect } from "next/navigation";
import { useState } from "react";

export default function CreateUser() {
    const [username, setUsername] = useState<string>('');
    const [password, setPassword] = useState<string>('');
    const [msg, setMsg] = useState<string>('');
    const create = (e: Event) => {
        e.preventDefault();
        fetch('/auth/createuser', {
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
                if (result.result == "ok") {
                    redirect('/');
                } else {
                    setMsg("Error");
                }
            });
    }


    return (<>
        <div className="grid grid-rows-[20px_1fr_20px] items-cente
  r justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
            <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
                <h1>Create a new User</h1>
                <h3>{msg}</h3>
                <form onSubmit={create} >
                    <input type="text" placeholder="Name" className="input input-bordered w-full max-w-xs" value={username} onChange={(e) => setUsername(e.target.value)} />
                    <input type="password" placeholder="Password" className="input input-bordered w-full max-w-xs" value={password} onChange={(e) => setPassword(e.target.value)} />
                    <button className="btn" type="submit">Login</button>
                </form>
            </main >
        </div >
    </>
    );
}