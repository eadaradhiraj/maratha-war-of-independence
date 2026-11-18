# Maratha: War of Independence &bull; Grand Campaign (1700–1750 A.D.)

A WebAssembly Grand Strategy game set in the Indian Subcontinent during the year 1700 A.D., depicting the climactic phase of the 27 Years' War. Take command of the **Maratha Confederacy** under Regent **Maharani Tarabai I** to liberate the subcontinent from the aging Mughal Emperor **Aurangzeb (Alamgir I)** and establish Hindavi Swarajya.

Built entirely with **Rust + WebAssembly (Macroquad / WebGL)** with zero external web dependencies and a lightweight ~680 KB binary footprint.

---

## Features

### 1. Authentic Historical India Theater (18 Provinces)
- **Maratha Strongholds**: Bijapur (*Satara* - Maratha Capital), Carnatica (*Arcot*).
- **Mughal Hegemony**: Hindustan (*Akbarabad / Delhi* - Imperial Capital), Punjab (*Lahore*), Bengal (*Calcutta*), Gujarat (*Ahmedabad*), Malwa (*Ujjain*), Berar (*Nagpur*), Ahmadnagar (*Ahmadnagar*), Sindh (*Neroon Kot*), Orissa & Circars (*Cuttack*), Hyderabad (*Hyderabad*), Rajpootana (*Udaipur*).
- **Independent & European Enclaves**: Kingdom of Portugal (*Goa Velha*), Sultanate of Mysore (*Mysore*), Ceylon (*Trincomalee*), Kashmir (*Srinagar*), Afghanistan (*Kabul*).

### 2. Campaign Map & Visual Presentation
- **Dynamic 2D Province Vector Engine**: Interactive polygon boundaries, coastal water rendering (Arabian Sea, Bay of Bengal, Indian Ocean), capital fortresses, and commercial sea anchorages.
- **Controls**:
  - **Left Click**: Select province and view regional statistics, public order, and garrisons.
  - **Right Click / Drag**: Pan camera across the subcontinent (or use WASD / Arrow Keys).
  - **Mouse Wheel**: Smooth zoom into provinces or out to the strategic theater level.
  - **Right Click Neighbor Province**: Order selected army to march or launch an assault.
  - **Spacebar / END TURN**: Advance the seasonal turn cycle (Summer / Winter 1700+).

### 3. Economy & Provincial Administration
- **Bespoke Tax Tiers**: Exempt, Low, Normal, High, Extortionate with dynamic public order and revenue trade-offs.
- **Infrastructure Projects**:
  - *Hill Fortress (Gadh)*: Public order boost and heavy defensive garrison.
  - *Regimental Barracks*: Enables training of line infantry, matchlocks, and cavalry.
  - *Bazaar & Trade Guild*: Regional tax and economic growth booster.
  - *Deepwater Trading Port*: High-yield maritime revenue.
  - *Temple / Mosque*: Culture stabilization and unrest mitigation.

### 4. Tactical Auto-Resolve Simulation
- Three-phase battlefield resolution:
  1. **Artillery Barrage & Skirmish Lines**: Topchi bronze guns and long-range Jezail skirmishers trade opening fire.
  2. **Rolling Musket Volleys**: Sepoy and matchlock lines trade volleys with discipline checks.
  3. **Cavalry Charges & Flanks**: Bargir and Silhadar lancers sweep across flanks with drawn talwars against defenders.
- Detailed battle reports with troop counts, casualty breakdown, and victory announcements.

### 5. Government & Technology
- **Council of Ministers (Cabinet)**:
  - *Head of State*: Maharani Tarabai I (+15% Battle Morale)
  - *Peshwa / Grand Vizier*: Ramchandra Pant Amatya (+10% Regional Growth)
  - *Pant Amatya (Treasury)*: Shankraji Narayan (-10% Upkeep Costs)
  - *Senapati (War)*: Dhanaji Jadhav (+2 Campaign Movement)
  - *Nyayadhish (Justice)*: Honaji Pant (+2 Public Order)
- **18th-Century Tech Innovations**:
  - Ring Bayonets, Canister Shot, Carbine Drills, Cadastral Land Surveys, Fly Shuttle Weaving Mills, and Hindavi Swarajya Charters.

---

## Quickstart & How to Play

### 1. Launch the Local Web Server
```bash
./run_web.sh
```
Open **`http://localhost:8080`** in any modern web browser (Chrome, Firefox, Safari, Edge).

### 2. Rebuilding the WASM Binary (Optional)
If you modify any Rust source code:
```bash
./build_wasm.sh
```
The compiled output is automatically copied to `web/game_engine.wasm` and `web/maratha_war_of_independence.wasm`.
