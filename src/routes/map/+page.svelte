<script lang="ts">
    import { Map, TileLayer, Marker, Popup } from "sveaflet";
    let userLocation: [number, number] = $state([0, 0]);
    if (navigator.geolocation) {
        navigator.geolocation.getCurrentPosition(
            (position) => {
                userLocation = [
                    position.coords.latitude,
                    position.coords.longitude,
                ];
                console.log("User location:", userLocation);
            },
            (error) => {
                console.error("Error getting location:", error);
            },
        );
    } else {
        console.error("Geolocation is not supported by this browser.");
    }
</script>

<div style="width:100%;height:100vh;">
    <Map
        options={{
            center: userLocation,
            zoom: 13,
        }}
    >
        <TileLayer url={"https://tile.openstreetmap.org/{z}/{x}/{y}.png"} />
        <Marker latLng={userLocation} />
    </Map>
</div>
