# bug: No audio output — PipeWire falling back to auto_null
2026-03-12  #bug #arch #pipewire #audio #hardware

## symptom
No audio output after booting from personal SSD on a new machine (HP Victus 16). PipeWire running but using a dummy sink:
```bash
pactl list sinks short
→ 34   auto_null   PipeWire   float32le 2ch 48000Hz   RUNNING
```
`aplay -l` only showed HDA NVidia HDMI ports — internal Realtek ALC245 was completely invisible.

## reproduce
Boot Arch on Intel Tiger Lake machine without `sof-firmware` installed.

## suspected cause
HP Victus 16 uses an **Intel Tiger Lake DSP** audio controller. The kernel's SOF driver claims the device but fails to load because `sof-firmware` is not installed — leaving the Realtek card in a broken/unclaimed state with nothing for PipeWire to attach to.

Evidence from `dmesg`:
```
sof-audio-pci-intel-tgl: SOF firmware and/or topology file not found.
sof-audio-pci-intel-tgl: Check if you have 'sof-firmware' package installed.
sof-audio-pci-intel-tgl: error: sof_probe_work failed err: -2
```

## tried
- Restarting PipeWire — no effect, still `auto_null`
- Checking `systemctl --user status pipewire` — service healthy, issue was upstream at driver level

## fix

### step 1 — install SOF firmware
```bash
yay -S sof-firmware
sudo reboot
```

After reboot, `aplay -l` showed:
```
card 1: sofhdadsp [sof-hda-dsp], device 0: HDA Analog
```

### step 2 — unmute channels in AlsaMixer
```bash
alsamixer
# Navigate to Speaker → press M to unmute → arrow Up to 100
# Navigate to Headphone → arrow Up to desired level
# Press Esc to exit

sudo alsactl store   # persist levels across reboots
```

### step 3 — verify PipeWire picked up real sink
```bash
pactl list sinks short
# Should show alsa_output.pci-... not auto_null
# SUSPENDED is normal when nothing is playing
```

### alternative fix (if sof-firmware doesn't work)
Force legacy HDA driver instead of SOF:
```bash
sudo nvim /etc/modprobe.d/alsa.conf
# Add: options snd-intel-dspcfg dsp_driver=1

sudo mkinitcpio -P
sudo reboot
```

---

## quick reference

### diagnosis
```bash
pactl list sinks short                              # auto_null = bad, real sink = good
aplay -l                                            # list ALSA hardware devices
dmesg | grep -i "sof\|hda\|audio" | tail -20       # kernel audio boot messages
lsmod | grep snd_hda                                # which HDA modules are loaded
```

### PipeWire
```bash
systemctl --user status pipewire pipewire-pulse wireplumber
systemctl --user restart pipewire pipewire-pulse wireplumber
pactl set-sink-volume @DEFAULT_SINK@ 50%
pactl set-sink-mute @DEFAULT_SINK@ toggle
```

### ALSA
```bash
alsamixer              # interactive TUI mixer
alsamixer -c 1         # open mixer for card index 1
sudo alsactl store     # save mixer levels
speaker-test -c 2      # white noise test
```

### AlsaMixer keys
| Key | Action |
|---|---|
| `M` | Toggle mute |
| `↑ / ↓` | Adjust volume |
| `F3/F4/F5` | Playback / Capture / All channels |
| `F6` | Select sound card |
| `Esc` | Exit |

## links
- [[arch-linux]]
