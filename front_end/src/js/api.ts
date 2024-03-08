import { push } from "svelte-spa-router";
import { type Group, type GroupResponse, type User } from "./types";
import {
    ENDPOINT_EXPENSES,
    ENDPOINT_GET_USERS,
    ENDPOINT_GROUP,
    ENDPOINT_GROUPS,
    ENDPOINT_SESSION_LOGOUT,
    ENDPOINT_USER,
} from "./endpoints";

/// get all the groups
export async function getGroups(): Promise<Array<Group>> {
    const response = await fetch(ENDPOINT_GROUPS);
    const groups = await response.json();
    return groups;
}

/// get a group for a specific group_id
export async function getGroup(group_id: number): Promise<GroupResponse> {
    const response = await fetch(ENDPOINT_GROUP + group_id + "/");
    const group: GroupResponse = await response.json();
    return group;
}

/// gets the current user
export async function getUser() {
    const response = await fetch(ENDPOINT_USER);
    const user_js = await response.json();
    return user_js;
}
/// get all users
export async function getUsers(): Promise<Array<User>> {
    const response = await fetch(ENDPOINT_GET_USERS);
    const users = await response.json();
    return users;
}

/// prefetch all the users groups
export async function prefetchGroups(g: Array<Group>) {
    for (const i of g) {
        await fetch(ENDPOINT_EXPENSES + i.id + "/");
    }
}

export async function logout() {
    await fetch(ENDPOINT_SESSION_LOGOUT);
    push("/");
}