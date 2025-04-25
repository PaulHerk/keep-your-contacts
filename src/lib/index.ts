// let url = "https://keep-your-contacts-fkfz.shuttle.app/persons"

import { error, redirect } from "@sveltejs/kit";
import { api_url } from "../routes/state.svelte";
import { goto } from "$app/navigation";

async function request(url: string, options: RequestInit = {}) {
  const fetchOptions: RequestInit = {
    credentials: "include",
    headers: {
      "Content-Type": "application/json", // Form request use svelte's fetch function that includes credentials?
      ...options.headers,
    },
    ...options,
  };
  try {
    const response = await fetch(url, fetchOptions);
    if (response.ok) {
      return response;
    } else {
      if (response.status === 401) {
        console.error("Unauthorized aa", await response.text());
        error(401);
      } else if (response.status === 500) {
        console.error("500:", response.text.toString());
        error(500);
      } else {
        error(response.status);
      }
    }
  } catch (e) {
    console.error("Api not responding correctly. aa", e);
    error(500);
  }
}

// // place files you want to import through the `$lib` alias in this folder.
// export let add_person = async (newPerson: NewPerson) => {
//     const response = await fetch(`${api_url}/persons`, {
//         method: "POST",
//         headers: {
//             "Content-Type": "application/json",
//         },
//         body: JSON.stringify({ ...newPerson }),
//     });
//     if (!response.ok) {
//         throw new Error("Network response was not ok");
//     }
//     // response.body?.getReader().read().then((data) => console.log(data));
//     return await response.json() as Person;
// }

// // id starts at 0, but is increased by 1 in the backend
// export let get_persons = async () => {
//     const response = await fetch(`${api_url}/persons?page=1&per_page=2`);
//     if (!response.ok) throw new Error("Network response was not ok" + response.status);
//     return await response.json() as Person[];
// }

// export let delete_person = async (id: number) => {
//     const response = await fetch(`${api_url}/persons/${id}`, {
//         method: "DELETE",
//     });
//     if (!response.ok) {
//         throw new Error("Network response was not ok");
//     }
//     return id
// }

export const api_get = async (url: string, options: RequestInit) => {
  return request(url, { method: "GET", ...options });
};
export const api_post = (url: string, body: object, options: RequestInit) => {
  return request(url, {
    method: "POST",
    body: JSON.stringify(body),
    ...options,
  });
};
