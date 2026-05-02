export enum IntervalMode {
  Random = 0,
  Fixed = 1,
}

export interface FixedIntervalOptions {
  hours: number;
  minutes: number;
  seconds: number;
  milliseconds: number;
}

export interface RandomIntervalOptions {
  min: number;
  max: number;
}

export enum RepeatMode {
  Forever = 0,
  Count = 1,
}

export enum ClickButton {
  Left = 0,
  Middle = 1,
  Right = 2,
}

export enum ClickQuantity {
  Single = 0,
  Double = 1,
}

export interface MousePos {
  x: number;
  y: number;
}

export type Settings = {
  interval_mode: IntervalMode;
  interval_fixed_options: FixedIntervalOptions;
  interval_rand_options: RandomIntervalOptions;

  repeat_mode: RepeatMode;
  repeat_count: number;

  click_button: ClickButton;
  click_quantity: ClickQuantity;

  set_mouse_position: boolean;
  mouse_position: MousePos;
}

export const interval_mode_to_string = (mode: IntervalMode): string => {
  return mode === IntervalMode.Random ? "random" : "fixed";
};

export const string_to_interval_mode = (str: string): IntervalMode => {
  return str === "random" ? IntervalMode.Random : IntervalMode.Fixed;
};

export const repeat_mode_to_string = (mode: RepeatMode): string => {
  return mode === RepeatMode.Forever ? "forever" : "count";
};

export const string_to_repeat_mode = (str: string): RepeatMode => {
  return str === "forever" ? RepeatMode.Forever : RepeatMode.Count;
};

export const click_button_to_string = (button: ClickButton): string => {
  switch (button) {
    case ClickButton.Left:
      return "left";
    case ClickButton.Middle:
      return "middle";
    case ClickButton.Right:
      return "right";
  }
};

export const string_to_click_button = (str: string): ClickButton => {
  switch (str) {
    case "middle":
      return ClickButton.Middle;
    case "right":
      return ClickButton.Right;
    default:
      return ClickButton.Left;
  }
};

export const click_quantity_to_string = (quantity: ClickQuantity): string => {
  return quantity === ClickQuantity.Single ? "single" : "double";
};

export const string_to_click_quantity = (str: string): ClickQuantity => {
  return str === "double" ? ClickQuantity.Double : ClickQuantity.Single;
};
