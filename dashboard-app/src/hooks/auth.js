import { writable } from "svelte/store";
import { apiRequest } from "./common";

export const auth_store = writable({
    isLogged: false,
    id: 0,
    username: "",
});

export async function logout() {
    auth_store.set({
        isLogged: false,
        id: 0,
        username: ""
    });

    return apiRequest("get", "/user/logout", null);
}

export async function login(username, password) {

    return apiRequest("post", "/user/login", {
        username,
        password
    })
    .then((data) => {
        auth_store.set({
            isLogged: true,
            id: data.user_id,
            username
        });
        return Promise.resolve(data);
    })
    .catch((e) => {
        auth_store.set({
            isLogged: false,
            id: 0,
            username: ""
        });

        return Promise.reject(e);
    });
}

export async function register(username, password) {

    return apiRequest("post", "/user/register", {
        username,
        password
    })
    .then((data) => {
        return Promise.resolve(data);
    })
    .catch((e) => {
        return Promise.reject(e);
    });
}