<script lang="ts">
  import { Map, TileLayer, Marker, Popup, DivIcon } from "sveaflet";
  import { persons } from "../store";

  let userLocation: [number, number] | null = $state(null);

  let contactWithLocations = $persons
    .filter((p) => p.record !== null)
    .map((p) => {
      return {
        coord: [p.record?.lat as number, p.record?.lon as number], //how tf is p.record possibly null?????????
        name: p.firstName,
      };
    });
  if (navigator.geolocation) {
    navigator.geolocation.getCurrentPosition(
      (position) => {
        userLocation = [position.coords.latitude, position.coords.longitude];
      },
      (error) => {
        console.error("Error getting location:", error);
      }
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
    {#each contactWithLocations as contact}
      <Marker latLng={contact.coord}>
        <DivIcon>
          <div class="text-lg text-purple-600">
            {contact.name}
          </div>
        </DivIcon>
      </Marker>
    {/each}
  </Map>
</div>
