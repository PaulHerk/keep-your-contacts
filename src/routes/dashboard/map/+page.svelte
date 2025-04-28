<script lang="ts">
  import { Map, TileLayer, Marker, Tooltip, DivIcon } from "sveaflet";
  import { persons } from "../store";
  import { onMount } from "svelte";
  import { api_request } from "$lib";
  import { error } from "@sveltejs/kit";
  import { MapPin } from "@lucide/svelte";

  let userLocation: [number, number] | null = $state(null);

  let contactWithLocations = $state(
    $persons
      .filter((p) => p.record !== null)
      .map((p) => {
        return {
          coordinate: [p.record?.lat as number, p.record?.lon as number], //how tf is p.record possibly null?????????
          firstName: p.firstName,
          lastName: p.lastName,
        };
      })
  );

  onMount(async () => {
    api_request(fetch, "/persons/coordinates").then(async (response) => {
      if (!response.ok) error(500, await response.text());
      contactWithLocations = await response.json();
    });
  }); // TODO: put in load function

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
    <TileLayer
      url={"https://tile.openstreetmap.de/{z}/{x}/{y}.png"}
      options={{
        maxZoom: 18,
        attribution:
          '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
      }}
    />
    {#if userLocation}
      <Marker latLng={userLocation}>
        <Tooltip options={{ content: "Your Location." }} />
      </Marker>
    {/if}
    {#each contactWithLocations as contact}
      <Marker latLng={contact.coordinate}>
        <DivIcon options={{ className: "transparent", iconAnchor: [12.5, 25] }}>
          <div class="text-md text-purple-600">
            <MapPin size={25} />
            {contact.firstName}
          </div>
          <Tooltip
            options={{ content: `${contact.firstName} ${contact.lastName}` }}
          />
        </DivIcon>
      </Marker>
    {/each}
  </Map>
</div>
