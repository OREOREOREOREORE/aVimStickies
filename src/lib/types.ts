export type Note = {
  id: string;
  title: string;
  content: string;
  pinned: boolean;
  color: string;
};

export type Settings = {
  font_family: string;
  font_size: number;
  theme: string;
  opacity: number;
  show_preview_button: boolean;
  show_action_buttons: boolean;
  enable_color_cycle: boolean;
  show_status_bar: boolean;
  show_line_numbers: boolean;
  wrap_text: boolean;
};
