// let url = "https://keep-your-contacts-fkfz.shuttle.app/persons"

import { error, redirect } from "@sveltejs/kit";
import { goto } from "$app/navigation";

async function request(
  fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
  url: string,
  options: RequestInit = {}
) {
  const fetchOptions: RequestInit = {
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      ...options.headers,
    },
    ...options,
  };
  try {
    const response: Response = await fetch(url, fetchOptions);
    if (response.ok) {
      return response;
    } else {
      if (response.status === 401) {
        error(401, await response.text());
      } else if (response.status === 500) {
        error(500, "Error 500: " + (await response.text()));
      } else {
        error(response.status, "Unknown Error " + response.statusText);
      }
    }
  } catch (e) {
    error(500, "Api not responding correctly: " + e);
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

export const getCoordsFromPlace = async (query: string) => {
  let error = {
    success: false,
    placeNotFound: true,
    message: "",
    coordinate: undefined,
  };
  try {
    let res = await fetch(
      `https://nominatim.openstreetmap.org/search?q=${query}&limit=1&format=jsonv2`
    );
    if (res.ok) {
      let place = (await res.json())[0];
      if (!place) return { ...error, message: "This place was not found." };
      let coordinate: Coordinate = {
        lat: parseFloat(place.lat),
        lon: parseFloat(place.lon),
      };
      return { success: true, coordinate: coordinate };
    }
    error.message = "Error with the api: " + res.statusText;
  } catch (e) {
    error.message =
      "Error with the api: " + e + "Do you have internet connection?";
  }
  return error;
};

// export const api_url = "http://localhost:8000";
export const api_url = "https://keep-your-contacts-fkfz.shuttle.app";
export const api_request = async (
  fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
  url: string,
  options: RequestInit = {}
) => {
  return await request(fetch, api_url + url, options);
};
