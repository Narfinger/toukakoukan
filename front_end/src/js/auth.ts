import { push } from 'svelte-spa-router';
import { ENDPOINT_SESSION_AUTH, ENDPOINT_SESSION_LOGIN, ENDPOINT_SESSION_LOGOUT } from './endpoints.js';
import { user } from './store.js';
import type { LoginResult } from './types.js';

export async function getSession() {
    console.log("Getting session");
    fetch(ENDPOINT_SESSION_AUTH, { credentials: 'include' }).then(async function (res) {
        if (res.ok) {
            const sessionResponse = await res.json();
            if (sessionResponse.user_id !== '') {
                user.set(sessionResponse.user_id);
            } else {
                user.set('');
                push("/");
            }
        }
    }).catch(function () {
        console.log("some error in getting session, don't worry about it");
        push("/");
    });
}

export async function postLogin(username: string, password: string): Promise<boolean> {
    return await fetch(ENDPOINT_SESSION_LOGIN, {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ username: username, password: password }),
    }).then((r) => r.json()).then((r: LoginResult) => r.result == "ok").catch(() => false);
}

export async function getLogout() {
    const res = await fetch(ENDPOINT_SESSION_LOGOUT, { credentials: 'include' });

    const logoutResponse = await res.json();
    if (logoutResponse.result == "error") {
        // may want to return an error here
    } else {
        user.set('');
    }
}