#!/usr/bin/env python3

import tkinter as tk
from tkinter import filedialog, messagebox, ttk
import json
import os
import locale
import sys
import random


class SaveEditor:
    def __init__(self, root):
        self.root = root
        self.current_lang = self.detect_system_language()
        self.translations = self.get_translations()
        self.root.geometry("700x600")

        self.file_path = None
        self.save_data = None
        self.game_data = None
        self.content_data = None

        self.file_encrypted = True
        self.xor_mode = 'char'

        self.setup_ui()
        self.apply_language()

    def setup_ui(self):
        t = self.translations[self.current_lang]

        file_frame = ttk.Frame(self.root, padding="10")
        file_frame.pack(fill=tk.X)

        ttk.Button(file_frame, text=t['open_file'], command=self.open_file).pack(side=tk.LEFT, padx=5)
        self.file_label = ttk.Label(file_frame, text=t['file_not_selected'])
        self.file_label.pack(side=tk.LEFT, padx=5)

        game_frame = ttk.LabelFrame(self.root, text=t['game_data'], padding="10")
        game_frame.pack(fill=tk.X, padx=10, pady=5)

        ttk.Label(game_frame, text=t['money'] + ":").grid(row=0, column=0, sticky=tk.W, pady=2)
        self.money_var = tk.StringVar()
        ttk.Entry(game_frame, textvariable=self.money_var, width=20).grid(row=0, column=1, sticky=tk.W, pady=2)

        ttk.Label(game_frame, text=t['playtime'] + ":").grid(row=1, column=0, sticky=tk.W, pady=2)
        self.playtime_var = tk.StringVar()
        ttk.Entry(game_frame, textvariable=self.playtime_var, width=20).grid(row=1, column=1, sticky=tk.W, pady=2)

        ttk.Label(game_frame, text=t['room_name'] + ":").grid(row=2, column=0, sticky=tk.W, pady=2)
        self.roomname_var = tk.StringVar()
        ttk.Entry(game_frame, textvariable=self.roomname_var, width=20).grid(row=2, column=1, sticky=tk.W, pady=2)

        player_frame = ttk.LabelFrame(self.root, text=t['player_position'], padding="10")
        player_frame.pack(fill=tk.X, padx=10, pady=5)

        ttk.Label(player_frame, text=t['x'] + ":").grid(row=0, column=0, sticky=tk.W, pady=2)
        self.player_x_var = tk.StringVar()
        ttk.Entry(player_frame, textvariable=self.player_x_var, width=10).grid(row=0, column=1, sticky=tk.W, pady=2)

        ttk.Label(player_frame, text=t['y'] + ":").grid(row=0, column=2, sticky=tk.W, pady=2)
        self.player_y_var = tk.StringVar()
        ttk.Entry(player_frame, textvariable=self.player_y_var, width=10).grid(row=0, column=3, sticky=tk.W, pady=2)

        ttk.Label(player_frame, text=t['z'] + ":").grid(row=0, column=4, sticky=tk.W, pady=2)
        self.player_z_var = tk.StringVar()
        ttk.Entry(player_frame, textvariable=self.player_z_var, width=10).grid(row=0, column=5, sticky=tk.W, pady=2)

        items_frame = ttk.LabelFrame(self.root, text=t['add_objects'], padding="10")
        items_frame.pack(fill=tk.X, padx=10, pady=5)

        self.available_items = {
            "Pillow": "Pillow",
            "Cube": "Cube",
            "RTX4080Ti": "RTX4080Ti",
            "Projector": "Projector"
        }

        ttk.Label(items_frame, text=t['select_object'] + ":").grid(row=0, column=0, sticky=tk.W, pady=2)
        self.item_var = tk.StringVar()
        item_combo = ttk.Combobox(items_frame, textvariable=self.item_var,
                                  values=list(self.available_items.keys()), width=15)
        item_combo.grid(row=0, column=1, sticky=tk.W, pady=2)
        item_combo.current(0)

        ttk.Button(items_frame, text=t['add_object'], command=self.add_item).grid(row=0, column=2, padx=5)

        self.items_listbox = tk.Listbox(items_frame, height=5)
        self.items_listbox.grid(row=1, column=0, columnspan=3, sticky=tk.W + tk.E, pady=5)

        ttk.Button(items_frame, text=t['remove_selected'], command=self.remove_item).grid(row=2, column=0, columnspan=3, pady=2)

        button_frame = ttk.Frame(self.root, padding="10")
        button_frame.pack(fill=tk.X)

        ttk.Button(button_frame, text=t['save'], command=self.save_file).pack(side=tk.LEFT, padx=5)
        ttk.Button(button_frame, text=t['save_as'], command=self.save_as_file).pack(side=tk.LEFT, padx=5)

        self.status_label = ttk.Label(self.root, text=t['ready'], relief=tk.SUNKEN, anchor=tk.W)
        self.status_label.pack(fill=tk.X, side=tk.BOTTOM)

        self.added_items = []

    def detect_system_language(self):
        try:
            system_locale = locale.getlocale()[0]
            if system_locale:
                lang_code = system_locale.split('_')[0].lower()
                if lang_code in ['ru', 'en']:
                    return lang_code
        except Exception:
            pass

        for arg in sys.argv:
            if arg.lower() in ['--lang=en', '--en']:
                return 'en'
            elif arg.lower() in ['--lang=ru', '--ru']:
                return 'ru'

        return 'en'

    def get_translations(self):
        return {
            'ru': {
                'title': 'PC Simulator Save Editor',
                'open_file': 'Открыть файл',
                'file_not_selected': 'Файл не выбран',
                'select_file': 'Выберите файл сохранения',
                'file_types': [('All files', '*.*')],
                'game_data': 'GameData',
                'money': 'Деньги',
                'playtime': 'Playtime',
                'room_name': 'Имя сохранения',
                'player_position': 'Позиция игрока (для спавна объектов)',
                'x': 'X', 'y': 'Y', 'z': 'Z',
                'add_objects': 'Добавить объекты',
                'select_object': 'Выберите объект',
                'add_object': 'Добавить объект',
                'remove_selected': 'Удалить выбранный',
                'save': 'Сохранить',
                'save_as': 'Сохранить как...',
                'ready': 'Готов к работе',
                'file_loaded': 'Файл загружен успешно',
                'file_load_error': 'Ошибка загрузки',
                'object_added': 'Добавлен объект',
                'object_removed': 'Удален объект',
                'object_at_position': 'на позиции игрока',
                'save_success': 'Файл сохранён успешно',
                'save_error': 'Ошибка сохранения',
                'file_saved': 'Файл сохранён',
                'open_warning': 'Сначала откройте файл',
                'error_title': 'Ошибка',
                'success_title': 'Успех',
                'load_error': 'Не удалось загрузить файл',
                'save_error_msg': 'Не удалось сохранить файл',
                'invalid_format': 'Неверный формат файла',
                'unsupported_format': 'Неподдерживаемый формат файла. Расширение должно содержать "pc".'
            },
            'en': {
                'title': 'PC Simulator Save Editor',
                'open_file': 'Open file',
                'file_not_selected': 'File not selected',
                'select_file': 'Select save file',
                'file_types': [('All files', '*.*')],
                'game_data': 'GameData',
                'money': 'Money',
                'playtime': 'Playtime',
                'room_name': 'Save Name',
                'player_position': 'Player Position (for object spawn)',
                'x': 'X', 'y': 'Y', 'z': 'Z',
                'add_objects': 'Add Objects',
                'select_object': 'Select object',
                'add_object': 'Add object',
                'remove_selected': 'Remove selected',
                'save': 'Save',
                'save_as': 'Save as...',
                'ready': 'Ready to work',
                'file_loaded': 'File loaded successfully',
                'file_load_error': 'Load error',
                'object_added': 'Object added',
                'object_removed': 'Object removed',
                'object_at_position': 'at player position',
                'save_success': 'File saved successfully',
                'save_error': 'Save error',
                'file_saved': 'File saved',
                'open_warning': 'Open a file first',
                'error_title': 'Error',
                'success_title': 'Success',
                'load_error': 'Could not load file',
                'save_error_msg': 'Could not save file',
                'invalid_format': 'Invalid file format',
                'unsupported_format': 'Unsupported file format. Extension must contain "pc".'
            }
        }

    def apply_language(self):
        t = self.translations[self.current_lang]
        self.root.title(t['title'])
        self.status_label.config(text=t['ready'])

    def _xor_chars(self, text, key):
        return ''.join(chr(ord(c) ^ key) for c in text)

    def xor_decrypt(self, data):
        return self._xor_chars(data, 129)

    def xor_encrypt(self, data):
        return self._xor_chars(data, 129)

    def open_file(self):
        t = self.translations[self.current_lang]
        file_path = filedialog.askopenfilename(
            title=t['select_file'],
            filetypes=t['file_types']
        )
        if not file_path:
            return

        ext = os.path.splitext(file_path)[1].lower()
        if 'pc' not in ext:
            messagebox.showerror(t['error_title'], t['unsupported_format'])
            return

        self.file_path = file_path
        self.file_label.config(text=os.path.basename(file_path))
        self.load_file()

    def _parse_save_content(self, text):
        if not text:
            return None

        text = text.lstrip('\ufeff\x00\r\n\t ')

        idx = next((i for i, c in enumerate(text) if c == '{'), -1)
        if idx < 0:
            return None
        text = text[idx:]

        parts = text.split('\n', 1)
        if not parts or not parts[0].strip():
            return None

        try:
            game_data = json.loads(parts[0].strip())
        except (json.JSONDecodeError, ValueError):
            return None

        if not isinstance(game_data, dict):
            return None
        if not any(k in game_data for k in ('coin', 'playtime', 'roomName')):
            return None

        content_data = {"itemData": []}
        if len(parts) > 1 and parts[1].strip():
            try:
                content_data = json.loads(parts[1].strip())
            except (json.JSONDecodeError, ValueError):
                pass

        return game_data, content_data

    def load_file(self):
        t = self.translations[self.current_lang]
        try:
            with open(self.file_path, 'rb') as f:
                raw = f.read()

            if raw.startswith(b'\xef\xbb\xbf'):
                raw = raw[3:]

            variants = []

            try:
                text = raw.decode('utf-8')
                variants.append(('char-xor', self._xor_chars(text, 129)))
            except UnicodeDecodeError:
                pass

            try:
                dec = bytes(b ^ 129 for b in raw)
                variants.append(('byte-xor', dec.decode('utf-8')))
            except UnicodeDecodeError:
                pass

            try:
                variants.append(('plain', raw.decode('utf-8')))
            except UnicodeDecodeError:
                pass

            parsed = None
            chosen_name = None
            for name, text in variants:
                res = self._parse_save_content(text)
                if res is not None:
                    parsed = res
                    chosen_name = name
                    print(f"DEBUG: parsed as '{name}'")
                    break

            if parsed is None:
                preview = "\n".join(f"{n}: {v[:60]!r}" for n, v in variants) or "(нет вариантов)"
                raise ValueError(f"{t['invalid_format']}.\n{preview}")

            self.game_data, self.content_data = parsed

            if chosen_name == 'char-xor':
                self.file_encrypted = True
                self.xor_mode = 'char'
            elif chosen_name == 'byte-xor':
                self.file_encrypted = True
                self.xor_mode = 'byte'
            else:
                self.file_encrypted = False
                self.xor_mode = None

            self.money_var.set(str(self.game_data.get('coin', 0)))
            self.playtime_var.set(str(self.game_data.get('playtime', 0)))
            self.roomname_var.set(self.game_data.get('roomName', ''))

            pd = self.content_data.get('playerData') if isinstance(self.content_data, dict) else None
            if pd:
                self.player_x_var.set(str(pd.get('x', 0)))
                self.player_y_var.set(str(pd.get('y', 0)))
                self.player_z_var.set(str(pd.get('z', 0)))
            else:
                self.player_x_var.set("0")
                self.player_y_var.set("0")
                self.player_z_var.set("0")

            self.added_items = []
            self.items_listbox.delete(0, tk.END)
            for item in self.content_data.get('itemData', []):
                if 'spawnId' in item:
                    self.added_items.append(item)
                    self.items_listbox.insert(tk.END, f"{item['spawnId']} (ID: {item.get('id', 0)})")

            self.status_label.config(text=t['file_loaded'])

        except Exception as e:
            messagebox.showerror(t['error_title'], f"{t['load_error']}: {str(e)}")
            self.status_label.config(text=t['file_load_error'])

    def add_item(self):
        t = self.translations[self.current_lang]
        if not self.content_data:
            messagebox.showwarning(t['error_title'], t['open_warning'])
            return

        item_name = self.item_var.get()
        spawn_id = self.available_items.get(item_name)
        if not spawn_id:
            return

        item_id = random.randint(-2147483648, 2147483647)

        try:
            player_x = float(self.player_x_var.get() or 0)
            player_y = float(self.player_y_var.get() or 0)
            player_z = float(self.player_z_var.get() or 0)
        except ValueError:
            player_x, player_y, player_z = 0.0, 0.0, 0.0

        new_item = {
            "spawnId": spawn_id,
            "id": item_id,
            "pos": {"x": player_x, "y": player_y, "z": player_z},
            "rot": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}
        }

        self.added_items.append(new_item)
        self.items_listbox.insert(tk.END, f"{spawn_id} (ID: {item_id}) [{player_x:.1f}, {player_y:.1f}, {player_z:.1f}]")

        self.status_label.config(text=f"{t['object_added']}: {spawn_id} {t['object_at_position']}")
        print(f"DEBUG: Added item: {spawn_id}, ID: {item_id}, Position: {player_x}, {player_y}, {player_z}")

    def remove_item(self):
        t = self.translations[self.current_lang]
        selection = self.items_listbox.curselection()
        if selection:
            index = selection[0]
            if 0 <= index < len(self.added_items):
                removed_item = self.added_items.pop(index)
                self.items_listbox.delete(index)
                self.status_label.config(text=f"{t['object_removed']}: {removed_item['spawnId']}")

    def save_file(self):
        t = self.translations[self.current_lang]
        if not self.file_path:
            messagebox.showwarning(t['error_title'], t['open_warning'])
            return
        self.save_to_file(self.file_path)

    def save_as_file(self):
        t = self.translations[self.current_lang]
        file_path = filedialog.asksaveasfilename(
            title=t['save_as'],
            defaultextension=".pc",
            filetypes=t['file_types']
        )
        if file_path:
            self.save_to_file(file_path)

    def save_to_file(self, file_path):
        try:
            self.game_data['coin'] = int(float(self.money_var.get() or 0))
            self.game_data['playtime'] = float(self.playtime_var.get() or 0)
            self.game_data['roomName'] = self.roomname_var.get()

            if 'itemData' not in self.content_data:
                self.content_data['itemData'] = []

            for item in self.added_items:
                exists = any(existing.get('id') == item['id']
                             for existing in self.content_data['itemData'])
                if not exists:
                    if item.get('data') is None and 'data' in item:
                        del item['data']
                    self.content_data['itemData'].append(item)

            game_data_json = json.dumps(self.game_data, ensure_ascii=False)
            content_json = json.dumps(self.content_data, ensure_ascii=False)
            combined = game_data_json + "\n" + content_json

            if not getattr(self, 'file_encrypted', True):
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(combined)

            elif getattr(self, 'xor_mode', 'char') == 'byte':
                try:
                    if any(ord(c) > 255 for c in combined):
                        raise ValueError("non-byte chars")
                    data = bytes((ord(c) ^ 129) & 0xFF for c in combined)
                    with open(file_path, 'wb') as f:
                        f.write(data)
                except Exception:
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(self.xor_encrypt(combined))

            else:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(self.xor_encrypt(combined))

            t = self.translations[self.current_lang]
            self.status_label.config(text=f"{t['file_saved']}: {os.path.basename(file_path)}")
            messagebox.showinfo(t['success_title'], t['save_success'])

        except Exception as e:
            t = self.translations[self.current_lang]
            messagebox.showerror(t['error_title'], f"{t['save_error_msg']}: {str(e)}")
            self.status_label.config(text=t['save_error'])


if __name__ == "__main__":
    root = tk.Tk()
    app = SaveEditor(root)
    root.mainloop()