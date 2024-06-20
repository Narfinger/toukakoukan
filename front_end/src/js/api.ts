import { push } from "svelte-spa-router";
import { type Group, type GroupResponse, type User } from "./types";
import {
    ENDPOINT_EXPENSES,
    ENDPOINT_GET_USERS,
    ENDPOINT_GROUP,
    ENDPOINT_GROUPS,
    ENDPOINT_SESSION_LOGOUT,
    ENDPOINT_USER,
    ENDPOINT_USER_CREATION_ENABLED,
} from "./endpoints";

/// get all the groups
export async function getGroups(): Promise<Array<Group>> {
    return await fetch(ENDPOINT_GROUPS, { credentials: 'include' }).then((res) => res.json());
}

/// get a group for a specific group_id
export async function getGroup(group_id: number): Promise<GroupResponse> {
    const response = await fetch(ENDPOINT_GROUP + group_id + "/", { credentials: 'include' });
    const group: GroupResponse = await response.json();
    return group;
}

/// gets the current user
export async function getUser(): Promise<User> {
    const response = await fetch(ENDPOINT_USER, { credentials: 'include' });
    const user_js = await response.json();
    return user_js;
}
/// get all users
export async function getUsers(): Promise<Array<User>> {
    const response = await fetch(ENDPOINT_GET_USERS, { credentials: 'include' });
    const users = await response.json();
    return users;
}

/// prefetch all the users groups
export async function prefetchGroups(g: Array<Group>) {
    for (const i of g) {
        await fetch(ENDPOINT_EXPENSES + i.id + "/", { credentials: 'include' });
    }
}

// logout
export async function logout() {
    await fetch(ENDPOINT_SESSION_LOGOUT, { credentials: 'include' });
    push("/");
}

export async function user_creation_enabled(): Promise<boolean> {
    const res = await fetch(ENDPOINT_USER_CREATION_ENABLED, { credentials: 'include' });
    const r = await res.json();
    return r;
}