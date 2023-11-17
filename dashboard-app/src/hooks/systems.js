import { apiRequest } from "./common";

export const system = {
    name: "",
    description: ""
}

export async function getSystemListByUsers() {
    return apiRequest("get", `/system/list`, null)
    .then((systems) => {

        return apiRequest("get", `/user/list`, null)
        .then((users) => {
            return Promise.resolve(users.map((user) => {
                return {
                    ...user,
                    systems: systems.filter((system) => system.owner_id === user.user_id)
                } 
            }));
        })
        .catch((e) => {
            return Promise.reject(e);
        })

    })
    .catch((e) => {
        return Promise.reject(e);
    })
}

export async function getSystemList() {
    return apiRequest("get", `/system/list`, null);
}

export async function addSystem(new_system) {
    return apiRequest("post", "/system", new_system);
}

export async function deleteSystem(system_id) {
    return apiRequest("delete", `/system/${system_id}`, null);
}

export async function updateSystem(updated_system, system_id) {
    return apiRequest("patch", `/system/${system_id}`, updated_system)
    .then((data) => {
        return Promise.resolve(data);
    })
    .catch((e) => {
        return Promise.reject(e);
    })
}

export async function getUsersInSystem(system_id) {
    return apiRequest("get", `/system/${system_id}/user/list`, null);
}

export async function addUserToSystem(system_id, user_id) {
    return apiRequest("post", `/system/${system_id}/user/${user_id}`, null);
}

export async function deleteUserFromSystem(system_id, user_id) {
    return apiRequest("delete", `/system/${system_id}/user/${user_id}`, null);
}
