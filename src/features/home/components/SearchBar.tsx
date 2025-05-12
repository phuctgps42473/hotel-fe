'use client';

import { searchList } from '@/seeds/features/home';
import { House, Minus, Plus, Search, X } from 'lucide-react';
import React, { useState, useRef, useEffect, useReducer } from 'react';

type GuestCounts = {
  adults: number;
  children: number;
  infants: number;
  pets: number;
};

type GuestCategory = keyof GuestCounts;

const guestCategories: { id: GuestCategory; label: string; description: string }[] = [
  { id: 'adults', label: 'Adults', description: 'Ages 13 or above' },
  { id: 'children', label: 'Children', description: 'Ages 2-12' },
  { id: 'infants', label: 'Infants', description: 'Under 2' },
  { id: 'pets', label: 'Pets', description: 'Bringing a service animal?' },
];

type SearchState = {
  location: string,
  checkInDate: string,
  checkOutDate: string,
  guests: GuestCounts
}

const INIT_STATE: SearchState = {
  location: "",
  checkInDate: "",
  checkOutDate: "",
  guests: {
    adults: 0,
    children: 0,
    infants: 0,
    pets: 0
  }
};

type Action = {
  type: "CHANGE_LOCATION" | "CHANGE_CHECK_IN_DATE" | "UPDATE_GUESTS" | "CHANGE_CHECK_OUT_DATE",
  payload: Partial<SearchState>
};

function searchReducer(state: SearchState, { type, payload }: Action): SearchState {
  console.log({ type, payload });
  switch (type) {
    case "CHANGE_LOCATION":
      return { ...state, location: payload.location! };
    case "CHANGE_CHECK_IN_DATE":
      return { ...state, checkInDate: payload.checkInDate! };
    case "CHANGE_CHECK_OUT_DATE":
      return { ...state, checkOutDate: payload.checkOutDate! };
    case "UPDATE_GUESTS":
      return { ...state, guests: { ...payload.guests! } };
    default:
      throw new Error(`ERROR: INVALID ACTION TYPE: ${type}`);
  }
}

export default function SearchBar() {
  const [searchState, dispatch] = useReducer(searchReducer, INIT_STATE);
  const { guests, checkInDate, checkOutDate, location } = searchState;

  const [locationSuggestions, setLocationSuggestions] = useState(searchList);
  const [isGuestDropdownOpen, setIsGuestDropdownOpen] = useState<boolean>(false);
  const [focusedInput, setFocusedInput] = useState<string | null>(null);

  const guestDropdownRef = useRef<HTMLDivElement>(null);
  const guestButtonRef = useRef<HTMLButtonElement>(null);

  function handleLocationChange(e: React.ChangeEvent<HTMLInputElement>) {
    dispatch({
      type: "CHANGE_LOCATION",
      payload: { location: typeof e === "string" ? e : e.target.value }
    });
  };

  function handleGuestChange(category: GuestCategory, increment: number) {
    let guests = searchState.guests;

    const newCount = guests[category] + increment;

    if (category !== "adults" && newCount > 0 && guests["adults"] === 0) {
      guests["adults"] = 1;
      guests = { ...guests, [category]: newCount }
    } else if (category === "adults" &&
      Object.keys(guests).some((key) => guests[key as GuestCategory] > 0 && key !== "adults")
    ) {
      if (newCount <= 0) {
        guests = { ...guests, adults: 1 }
      } else {
        guests = { ...guests, adults: newCount }
      }
    } else {
      guests = { ...guests, [category]: newCount }
    }

    dispatch({
      type: "UPDATE_GUESTS",
      payload: { guests }
    });
  };

  function handleSearch(e: React.FormEvent) {
    e.preventDefault();
    console.log('Search Submitted:', {
      location,
      checkInDate,
      checkOutDate,
      guests,
    });
    // TODO: Implement actual search logic (e.g., navigate to results page, API call)
  };

  function getTotalGuests(): number {
    return guests.adults + guests.children + guests.infants;
  };

  function getGuestSummary(): string {
    const total = getTotalGuests();
    if (total === 0 && guests.pets === 0) return 'Add guests';
    return `${total} guests` + (guests.pets > 0 ? `, ${guests.pets} Pet${guests.pets > 1 ? 's' : ''}` : '');
  }


  useEffect(function () {
    if (focusedInput !== "location") return;
    if (location.trim().length < 2 && focusedInput !== "location") return;

    const timeout = setTimeout(() => {
      console.log("Searching");
    }, 300);

    return function () {
      clearTimeout(timeout);
    }

  }, [focusedInput, location])

  // Close guest dropdown when clicking outside
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      // Guest Dropdown is visible &&
      // Click happens outside of it &&
      // Guest button is visible &&
      // Click happens outside of it
      if (
        guestDropdownRef.current &&
        !guestDropdownRef.current.contains(event.target as Node) &&
        guestButtonRef.current &&
        !guestButtonRef.current.contains(event.target as Node)
      ) {
        setIsGuestDropdownOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return function () {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);


  return (
    <div className="hidden md:block w-full max-w-4xl mx-auto my-4">
      <form
        onSubmit={handleSearch}
        className="
          bg-base-100 dark:bg-base-300 border border-base-300 dark:border-base-content/20
          rounded-full shadow-md
          flex flex-col md:flex-row items-center
        "
      >

        <div className={`relative z-50 flex-grow w-full md:w-auto ${focusedInput === 'location' ? 'bg-base-200 dark:bg-base-100/10 rounded-full' : ''} transition-colors duration-200`}>
          <label htmlFor="location-input" className="block text-xs font-bold px-6 pt-3 text-base-content/80 dark:text-base-content/60">Where</label>
          <input
            id="location-input"
            type="text"
            placeholder="Search destinations"
            value={location}
            onChange={handleLocationChange}
            onFocus={() => setFocusedInput('location')}
            onBlur={() => setFocusedInput(null)}
            className="
              input input-ghost w-full !outline-none !border-none
              h-auto pb-3 pt-1 px-6 text-sm font-normal bg-transparent
              placeholder-base-content/50 dark:placeholder-base-content/40
              focus:bg-transparent focus:ring-0 focus:border-transparent
            "
          />

          {focusedInput === "location" ? (
            <ul className="
            absolute block top-full left-0 right-0 bg-base-100 border rounded-lg shadow-lg mt-1 z-10
            dark:bg-base-200  border-base-300 dark:border-base-content/20
            ">
              {locationSuggestions.map(loc => <li
                onMouseDown={() => dispatch({ type: "CHANGE_LOCATION", payload: { location: loc.name } })}
                key={loc.name}
                className="
                bloc list-row p-2 m-2 rounded-sm hover:bg-gray-300
                bg-base-100 dark:bg-base-200 shadow-lg border border-base-300 dark:border-base-content/20
              ">
                <div
                  className="flex items-center justify-start gap-3">
                  <House size={32} />
                  <div>
                    <div className="text-xs uppercase font-semibold">{loc.name}</div>
                    <p className="list-col-wrap text-xs">
                      {loc.description}
                    </p>
                  </div>
                </div>
              </li>
              )}
            </ul>
          ) : null}

        </div>

        {/* Divider */}
        <div className="h-8 w-px bg-base-300 dark:bg-base-content/20 hidden md:block self-center"></div>
        <div className="w-full h-px bg-base-300 dark:bg-base-content/20 md:hidden"></div> {/* Horizontal divider for mobile */}


        {/* --- Check-in Date --- */}
        <div className={`relative w-full md:w-auto ${focusedInput === 'checkin' ? 'bg-base-200 dark:bg-base-100/10 rounded-full' : ''} transition-colors duration-200`}>
          <label htmlFor="checkin-date" className="block text-xs font-bold px-6 pt-3 text-base-content/80 dark:text-base-content/60">Check in</label>
          <input
            id="checkin-date"
            type="date"
            value={checkInDate}
            onChange={(e) => dispatch({ type: 'CHANGE_CHECK_IN_DATE', payload: { checkInDate: e.target.value } })}
            onFocus={() => setFocusedInput('checkin')}
            onBlur={() => setFocusedInput(null)}
            min={new Date().toISOString().split('T')[0]}
            max={checkOutDate.length > 0 ? checkOutDate : ""}
            className="
              input input-ghost w-full !outline-none !border-none appearance-none
              h-auto pb-3 pt-1 px-6 text-sm font-normal bg-transparent
              placeholder-base-content/50 dark:placeholder-base-content/40
              focus:bg-transparent focus:ring-0 focus:border-transparent
              [&::-webkit-calendar-picker-indicator]:opacity-80
              dark:[&::-webkit-calendar-picker-indicator]:invert-[50%] dark:[&::-webkit-calendar-picker-indicator]:brightness-150
            "
          />
        </div>

        {/* Divider */}
        <div className="h-8 w-px bg-base-300 dark:bg-base-content/20 hidden md:block self-center"></div>
        <div className="w-full h-px bg-base-300 dark:bg-base-content/20 md:hidden"></div> {/* Horizontal divider for mobile */}

        {/* --- Check-out Date --- */}
        <div className={`relative w-full md:w-auto ${focusedInput === 'checkout' ? 'bg-base-200 dark:bg-base-100/10 rounded-full' : ''} transition-colors duration-200`}>
          <label htmlFor="checkout-date" className="block text-xs font-bold px-6 pt-3 text-base-content/80 dark:text-base-content/60">Check out</label>
          <input
            id="checkout-date"
            type="date"
            value={checkOutDate}
            onChange={(e) => dispatch({ type: 'CHANGE_CHECK_OUT_DATE', payload: { checkOutDate: e.target.value } })}
            onFocus={() => setFocusedInput('checkout')}
            onBlur={() => setFocusedInput(null)}
            min={checkInDate || new Date().toISOString().split('T')[0]} // Min checkout is checkin or today
            max={new Date(new Date().getUTCFullYear() + 1, 12).toISOString().split('T')[0]}
            className="
              input input-ghost w-full !outline-none !border-none appearance-none
              h-auto pb-3 pt-1 px-6 text-sm font-normal bg-transparent
              placeholder-base-content/50 dark:placeholder-base-content/40
              focus:bg-transparent focus:ring-0 focus:border-transparent
              [&::-webkit-calendar-picker-indicator]:opacity-80
              dark:[&::-webkit-calendar-picker-indicator]:invert-[50%] dark:[&::-webkit-calendar-picker-indicator]:brightness-150
            "
            placeholder="Add dates" // Placeholder might not show reliably for type="date"
          />
        </div>

        {/* Divider */}
        <div className="h-8 w-px bg-base-300 dark:bg-base-content/20 hidden md:block self-center"></div>
        <div className="w-full h-px bg-base-300 dark:bg-base-content/20 md:hidden"></div> {/* Horizontal divider for mobile */}

        {/* --- Guests Input & Search Button Wrapper --- */}
        <div className={`relative flex w-full md:w-auto justify-between items-center pl-2 md:pl-0 ${focusedInput === 'guests' || isGuestDropdownOpen ? 'bg-base-200 dark:bg-base-100/10 rounded-full' : ''} transition-colors duration-200`}>
          {/* --- Guests Button --- */}
          <button
            ref={guestButtonRef}
            type="button" // Important: prevent form submission
            onClick={() => {
              setIsGuestDropdownOpen(!isGuestDropdownOpen);
              setFocusedInput(isGuestDropdownOpen ? null : 'guests'); // Set focus when opening
            }}
            className="
                flex-grow text-left h-full
                px-4 py-3 md:px-6
                hover:bg-transparent focus:bg-transparent focus:outline-none
                "
          >
            <span className="block text-xs font-bold text-base-content/80 dark:text-base-content/60">Who</span>
            <span className={`block text-sm truncate ${getTotalGuests() > 0 || guests.pets > 0 ? 'text-base-content dark:text-base-content' : 'text-base-content/50 dark:text-base-content/40'}`}>
              {getGuestSummary()}
            </span>
          </button>

          {/* --- Search Button --- */}
          <div className="pr-2 py-2">
            <button
              type="submit"
              className="
                btn btn-primary btn-circle btn-md md:btn-lg
                flex items-center justify-center
              "
              aria-label="Search"
            >
              <Search size={24} />
            </button>
          </div>


          {/* --- Guest Dropdown --- */}
          {isGuestDropdownOpen && (
            <div
              ref={guestDropdownRef}
              className="
                absolute top-full right-0 mt-2 w-80 md:w-96 z-20
                bg-base-100 dark:bg-base-200 rounded-xl shadow-lg border border-base-300 dark:border-base-content/20
                p-5
              "
              onClick={(e) => e.stopPropagation()}
            >
              <button
                type="button"
                onClick={() => setIsGuestDropdownOpen(false)}
                className="absolute top-2 right-2 btn btn-ghost btn-sm btn-circle"
                aria-label="Close guest selection"
              >
                <X />
              </button>

              <h3 className="text-lg font-semibold mb-4 text-base-content dark:text-base-content/90">Guests</h3>

              {guestCategories.map(({ id, label, description }) => (
                <div key={id} className="flex items-center justify-between py-3 border-b border-base-200 dark:border-base-content/10 last:border-b-0 select-none">
                  <div>
                    <div className="font-medium text-base-content dark:text-base-content/90">{label}</div>
                    <div className="text-sm text-base-content/70 dark:text-base-content/60">{description}</div>
                  </div>
                  <div className="flex items-center space-x-3">
                    <button
                      type="button"
                      onClick={() => handleGuestChange(id, -1)}
                      disabled={guests[id] === 0 || (id === 'adults' && guests[id] === 1 && (guests.children > 0 || guests.infants > 0 || guests.pets > 0))}
                      className="
                        btn btn-circle btn-outline btn-sm
                        disabled:opacity-50 disabled:cursor-not-allowed
                        border-base-content/30 hover:bg-base-content/10
                        dark:border-base-content/40 dark:hover:bg-base-content/20
                        text-base-content dark:text-base-content
                      "
                      aria-label={`Decrement ${label}`}
                    >
                      <Minus />
                    </button>
                    <span className="w-6 text-center font-medium text-base-content dark:text-base-content/90 tabular-nums">
                      {guests[id]}
                    </span>
                    <button
                      type="button"
                      onClick={() => handleGuestChange(id, 1)}
                      disabled={id === "pets" ? guests.pets === 5 : getTotalGuests() > 15}
                      className="
                        btn btn-circle btn-outline btn-sm
                        disabled:opacity-50 disabled:cursor-not-allowed
                         border-base-content/30 hover:bg-base-content/10
                        dark:border-base-content/40 dark:hover:bg-base-content/20
                        text-base-content dark:text-base-content
                      "
                      aria-label={`Increment ${label}`}
                    >
                      <Plus />
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </form >
    </div >
  );
};