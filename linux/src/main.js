const { invoke } = window.__TAURI__.core;

async function calculatedamage() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  const damage =  await invoke("calculatedamage", { gun: GunInput.value, bodypart:BodypartInput.value,vest: VestInput.value,helmet:HelmetInput.value,ammotype:AmmoTypeInput.value });
  const newdamage = damage.split("///")
  DamagePerShotOutput.innerHTML = `Damage per Shot: ${newdamage[0]}`
  ShotsToKillOutput.innerHTML = `Shots to Kill: ${newdamage[1]}`
  TimeToKillOutput.innerHTML = `Time to Kill to Kill: ${newdamage[2]}`
  DamagePerSecondOutput.innerHTML =  `Damage per Second: ${newdamage[3]}`
  FirerateOutput.innerHTML = `Shots per Minute: ${newdamage[4]}`
}
const DamagePerShotOutput = document.getElementById("damagepershotoutput")
const ShotsToKillOutput = document.getElementById("shotstokilloutput")
const TimeToKillOutput = document.getElementById("timetokilloutput")
const DamagePerSecondOutput = document.getElementById("damagepersecondoutput")
const FirerateOutput = document.getElementById("firerateoutput")

const GunInput = document.getElementById("gun")
const HelmetInput = document.getElementById("helmet")
const VestInput = document.getElementById("vest")
const BodypartInput = document.getElementById("bodypart")
const AmmoTypeInput = document.getElementById("ammotype")

GunInput.onchange = calculatedamage
HelmetInput.onchange = calculatedamage
VestInput.onchange = calculatedamage
BodypartInput.onchange = calculatedamage
AmmoTypeInput.onchange = calculatedamage