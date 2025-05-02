<script lang="ts">
  import { Map, TileLayer, Marker, Tooltip, DivIcon } from "sveaflet";
  import { authToken, persons } from "../store";
  import { onMount } from "svelte";
  import { api_request } from "$lib";
  import { error } from "@sveltejs/kit";
  import { MapPin } from "@lucide/svelte";
  import type { PageProps } from "../$types";
  import { Modal } from "@skeletonlabs/skeleton-svelte";
  import AddOrChangePersons from "../components/AddOrChangePersons.svelte";

  let { data, form }: PageProps = $props();

  let openState = $state(false);
  let userLocation: [number, number] | null = $state(null);
  let personCoordinateToAdd: Coordinate | undefined = $state(undefined);

  let contactWithLocations = $state(
    $persons
      .filter((p) => p.record)
      .map((p) => {
        return {
          id: p.id,
          coordinate: [p.record?.lat, p.record?.lon], //how tf is p.record possibly null?????????
          firstName: p.firstName,
          lastName: p.lastName,
        };
      })
  );

  onMount(async () => {
    if ($persons.length === data.personCount) return;
    api_request(fetch, "/persons/coordinates", {}, $authToken).then(
      async (response) => {
        if (!response.ok) error(500, await response.text());
        contactWithLocations = await response.json();
      }
    );
  }); // TODO: put in load function? Idk

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
  function onMapClick(e: any) {
    personCoordinateToAdd = { lat: e.latlng.lat, lon: e.latlng.lng };
    console.log(personCoordinateToAdd);

    openState = true;
  } // TODO: edit person on click at marker, add person when clicking anywhere on map
</script>

{#key userLocation}
  <div style="width:100%;height:100vh;">
    <Map
      options={{
        center: userLocation ?? [0, 0],
        zoom: userLocation ? 13 : 2,
      }}
      onclick={onMapClick}
    >
      <TileLayer
        url={"https://tile.openstreetmap.de/{z}/{x}/{y}.png"}
        options={{
          maxZoom: 20,
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
        <Marker latLng={contact.coordinate} onclick={onMapClick}>
          <DivIcon
            options={{ className: "transparent", iconAnchor: [12.5, 25] }}
          >
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
    <Modal
      open={openState}
      onOpenChange={(e) => (openState = e.open)}
      contentBase="card bg-surface-100-900 p-4 space-y-4 shadow-xl max-w-screen-sm"
      backdropClasses="backdrop-blur-sm"
    >
      {#snippet content()}
        <AddOrChangePersons
          {form}
          personCount={data.personCount}
          {personCoordinateToAdd}
          perPage={1}
          bind:openState
        />
      {/snippet}
    </Modal>
  </div>
{/key}
