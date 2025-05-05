#!.venv/bin/python3
import customtkinter as tk
import zmq

context = zmq.Context()
socket = context.socket(zmq.REQ)
socket.connect("tcp://localhost:5555")

def terminate(event):
    socket.send(b"exit")
    socket.close()
def get_data(value:str):
    socket.send(bytes(value, "utf-8"))
    return socket.recv().decode()

def get_inputs():
    hp = HpInput.get()
    if hp == "":
        hp = "100"
    gun = GunInput.get()
    if gun == "":
        gun = "0"
    return f"{gun}///{BodyPartInput.get()}///{VestInput.get()}///{HelmetInput.get()}///{AmmoInput.get()}///{hp}"

def update_gui(value:str):
    split_value = value.split("///")
    DPShLabel.configure(text='Damage Per Shot:  '+split_value[0])
    ShotsToKillLabel.configure(text='Shots To Kill:  '+split_value[1])
    TimeToKillLabel.configure(text='Time To Kill:  '+split_value[2])
    DPSLabel.configure(text='Damage Per Second:  '+split_value[3])
    ShotsPerMinuteLabel.configure(text='Shots Per Minute:  '+ split_value[4])
def id_page():
    window.geometry('1022x500')
    DamageFrame.pack_forget()
    id_label=tk.CTkLabel(IdFrame,text='Gun\n\n[    Rifle    ]\n\nAK Alpha: 0         AK5C: 1         AK-74: 2\nAKM: 3          AK74-U: 4          AS VAL: 5\nAug: 6          Famas: 7          G36K: 8\nG3A3: 9          Galil: 10          L85A2: 11\nM16: 12          M1 SASS: 13          M4: 14\nSCAR: 15          SG552: 16          SKS: 17\nSKS (auto): 18          Stoner: 19          VSS: 20\n\n[     SMGs     ]\n\nAgram: 21          Cx8 Storm: 22          Vector10mm: 23\nVector9mm: 24          Luty: 25          Mac 10: 26\nMac 11: 27          MAT-49: 28          MP40: 29\nMP5: 30          MP7: 31          MP9: 32\nP90: 33          PP Bizon: 34          PPSH: 35\nThompson: 36          UMP45: 37          The Fin Reaper: 38\nJW MPX: 39\n\n[   Handguns   ]\n\nColt 1911: 40          JW 2011: 41          Deagle: 42\nFN57: 43          Glock: 44          JW G34: 45\nUSP 45: 46          Makarov: 47          Ruger: 48\nTokarev: 49\n\n[      Bolt-Action Rifles     ]\n\nAWM: 50          Mosin: 51          M1903: 52\nHunter 85: 53\n\n[    Sniper-Rifles    ]\n\nBarret: 54          Dragunov: 55\n\n[       LMGs       ]\n\nBAR: 56          XM250: 57\n\n\n\nBodypart\n\nHead: 0          Neck:1          Torso: 2          Pelvis: 3          Upper Arms: 4\nUpper Legs: 5          Lower Arms: 6          Lower Legs: 7          Hands: 8\nFeet: 9\n\n\n\nVest\n\nNone/A22: 0          LABV: 1          JPC2: 2          VestB: 3\n6B102: 4          6B43: 5          R61: 6\n\n\n\nHelmet\n\nNone: 0          SPH-5: 1          SSh-68: 2          6B47: 3\nATE: 3          C1300: 3          Ronin: 4          Cultist Mask: 4\nKrtek Mask: 4          Altyn: 5          Collector Helmet: 5\n\n\n\nAmmo\n\nFMJ: 0     AP: 1',font=("",16))
    id_close_button=tk.CTkButton(IdFrame, text="Close", command=id_page_close)
    id_close_button.place(x=0,y=0)
    id_label.pack(pady=10)
    IdFrame.place(x=500,y=0)
def id_page_close():
    window.geometry('500x500')
    DamageFrame.place(x=0,y=0)
    IdFrame.place_forget()

window = tk.CTk()
window.geometry("500x500")
window.title("Ghosts of Tabor Calculator")
window.resizable(width=False, height=False)

IdFrame = tk.CTkScrollableFrame(window,width=500,height=500)
DamageFrame=tk.CTkFrame(window,width=500,height=500)
DamageFrame.pack_propagate(False)
DFLabel=tk.CTkLabel(DamageFrame,text="Damage Calculator",font=("arial",20)).pack()


InputGrid=tk.CTkFrame(DamageFrame)

GunInput=tk.CTkEntry(InputGrid,width=150,justify="center",placeholder_text="GunID")
GunInput.grid(row=0,column=0)

BodyPartInput=tk.CTkEntry(InputGrid,width=150,justify="center",placeholder_text="BodypartID")
BodyPartInput.grid(row=0,column=1)

VestInput=tk.CTkEntry(InputGrid,width=150,justify="center",placeholder_text="VestID")
VestInput.grid(row=0,column=2)

HelmetInput=tk.CTkEntry(InputGrid,width=150,justify="center",placeholder_text="HelmetID")
HelmetInput.grid(row=1,column=0)

AmmoInput=tk.CTkEntry(InputGrid,width=150,justify="center",placeholder_text="AmmoID")
AmmoInput.grid(row=1,column=1)

HpInput=tk.CTkEntry(InputGrid,width=150,justify="center",placeholder_text="HP")
HpInput.grid(row=1,column=2)

CalculateButton=tk.CTkButton(InputGrid,text="Calculate",command=lambda: update_gui(get_data(get_inputs())))
CalculateButton.grid(row=3,column=1)

IDiButton=tk.CTkButton(InputGrid, text="IDs?", command=id_page)
IDiButton.grid(row=3,column=0)

DPShLabel=tk.CTkLabel(DamageFrame,text="")
ShotsToKillLabel=tk.CTkLabel(DamageFrame,text="")
TimeToKillLabel=tk.CTkLabel(DamageFrame,text="")
DPSLabel=tk.CTkLabel(DamageFrame,text="")
ShotsPerMinuteLabel=tk.CTkLabel(DamageFrame,text="")

DamageFrame.place(x=0,y=0)

InputGrid.pack()

DPShLabel.pack()
ShotsToKillLabel.pack()
TimeToKillLabel.pack()
DPSLabel.pack()
ShotsPerMinuteLabel.pack()

window.bind("<Destroy>", terminate)

window.mainloop()