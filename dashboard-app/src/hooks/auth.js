import { writable } from "svelte/store";
import { apiRequest } from "./common";

const auth = {
    id: 0,
    username: "",
    isLoggedin: false
};

export const auth_store = writable(auth);

export async function checkAuth() {

    let new_auth = auth;

    apiRequest("get", "/user", null)
    .then((data) => {
        new_auth.id = data.user_id;
        new_auth.username = data.username;
        new_auth.isLoggedin = true;
    })
    .catch((e) => {
        new_auth.isLoggedin = false;
    })
    .finally(() => {
        auth_store.set(new_auth);
    })
}

export async function logout() {
    let new_auth = auth;
    new_auth.isLoggedin = false;

    return apiRequest("get", "/user/logout", null)
    .finally(() => {
        auth_store.set(new_auth);
    });
}

export async function login(username, password) {

    let new_auth = auth;

    return apiRequest("post", "/user/login", {
        username,
        password
    })
    .then((data) => {
        new_auth.id = data.user_id;
        new_auth.username = data.username;
        new_auth.isLoggedin = true;
        return Promise.resolve(data);
    })
    .catch((e) => {
        new_auth.isLoggedin = false;
        return Promise.reject(e);
    })
    .finally(() => {
        auth_store.set(new_auth);
    });
}

export async function register(username, password) {

    return apiRequest("post", "/user/register", {
        username,
        password
    });
}