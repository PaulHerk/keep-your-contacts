// place files you want to import through the `$lib` alias in this folder.
export let add_persons = async (first_name: String, last_name: String, city: String, note: String) => {
    const response = await fetch("http://localhost:8000/persons", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ first_name, last_name, city, note }),
    });
    if (!response.ok) {
        throw new Error("Network response was not ok");
    }
    // response.body?.getReader().read().then((data) => console.log(data));
    return await response.json();
}

interface Person {
    id: number;
    first_name: string;
    last_name: string;
    city: string;
    note: string;
}

// id starts at 0, but is increased by 1 in the backend
export let get_persons = async () => {
    const response = await fetch(`http://localhost:8000/persons`);
    if (!response.ok) {
        throw new Error("Network response was not ok");
    }
    return await response.json() as Person[];
}