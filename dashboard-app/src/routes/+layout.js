import { checkAuth } from "../hooks/auth";

export const prerender = true;
export const trailingSlash = 'always'; 
export const ssr = false;

checkAuth();