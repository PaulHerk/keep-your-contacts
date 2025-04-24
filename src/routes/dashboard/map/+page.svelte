<script lang="ts">
    import { Map, TileLayer, Marker, Popup } from "sveaflet";

    let userLocation: [number, number] | null = $state(null);
    if (navigator.geolocation) {
        navigator.geolocation.getCurrentPosition(
            (position) => {
                userLocation = [
                    position.coords.latitude,
                    position.coords.longitude,
                ];
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
            center: userLocation ? userLocation : [0, 0],
            zoom: 13,
        }}
    >
        <TileLayer url={"https://tile.openstreetmap.org/{z}/{x}/{y}.png"} />
        {#if userLocation}
            <Marker latLng={userLocation}>
                <Popup options={{ content: "Your Location." }} />
            </Marker>
        {/if}
    </Map>
</div>
