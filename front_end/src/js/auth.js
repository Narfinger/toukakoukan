import { ENDPOINT_SESSION_AUTH, ENDPOINT_SESSION_LOGIN, ENDPOINT_SESSION_LOGOUT } from './endpoints.js';
import { user } from './store.js';

export async function getSession() {
    console.log("Getting session");
    fetch(ENDPOINT_SESSION_AUTH, { credentials: 'same-origin' }).then(async function (res) {
        if (res.ok) {
            let sessionResponse = await res.json();
            if (sessionResponse.user_id !== '') {
                user.set(sessionResponse.user_id);
            } else {
                user.set('');
            }
        }
    }).catch(function (error) {
        console.log("some error in getting session, don't worry about it");
    });
}

export async function postLogin(username, password) {
    const res = await fetch(ENDPOINT_SESSION_LOGIN, {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ username: username, password: password }),
    });
    return await res.json();
}

export async function getLogout() {
    const res = await fetch(ENDPOINT_SESSION_LOGOUT, { credentials: 'same-origin' });

    let logoutResponse = await res.json();
    if (logoutResponse.result == "error") {
        // may want to return an error here
    } else {
        user.set('');
    }
}