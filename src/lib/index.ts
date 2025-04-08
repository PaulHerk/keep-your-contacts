// place files you want to import through the `$lib` alias in this folder.
export let add_person = async (newPerson: NewPerson) => {
    const response = await fetch("http://localhost:8000/persons", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ ...newPerson }),
    });
    if (!response.ok) {
        throw new Error("Network response was not ok");
    }
    // response.body?.getReader().read().then((data) => console.log(data));
    return await response.json() as Person;
}

// id starts at 0, but is increased by 1 in the backend
export let get_persons = async () => {
    const response = await fetch(`http://localhost:8000/persons`);
    if (!response.ok) {
        throw new Error("Network response was not ok");
    }
    return await response.json() as Person[];
}

export let delete_person = async (id: number) => {
    const response = await fetch(`http://localhost:8000/persons/delete-person/${id}`, {
        method: "DELETE",
    });
    if (!response.ok) {
        throw new Error("Network response was not ok");
    }
    return id
}