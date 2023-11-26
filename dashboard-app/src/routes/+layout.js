import { checkAuth } from "../hooks/auth";
import '@fortawesome/fontawesome-free/css/all.min.css'

export const prerender = true;
export const trailingSlash = 'always'; 
export const ssr = false;

checkAuth();