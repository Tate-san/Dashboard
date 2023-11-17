import { apiRequest } from "./common";

export async function getUserList() {
    return apiRequest("get", `/user/list`, null);
}
