// place files you want to import through the `$lib` alias in this folder.
export let put = async (note: String) => {
    const response = await fetch("http://localhost:8000/todos", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ note }),
    });
    if (!response.ok) {
        throw new Error("Network response was not ok");
    }
    return await response.json();
}