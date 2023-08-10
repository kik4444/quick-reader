/*
 *    This file is part of Quick Reader.
 *
 *    Quick Reader is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU General Public License as published by
 *    the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    Quick Reader is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU General Public License for more details.
 *
 *    You should have received a copy of the GNU General Public License
 *    along with Quick Reader.  If not, see <https://www.gnu.org/licenses/>.
 */

use phf::phf_map;

use super::Base::Theme;

pub(crate) static BUTTON_GRADIENT: phf::Map<&'static str, Theme> = phf_map! {
  "white" => Theme {
    background: "bg-white",
    color: "text-blue-gray-900",
    shadow: "shadow-md shadow-blue-gray-500/10",
    hover: "hover:shadow-lg hover:shadow-blue-gray-500/20",
    focus: "focus:opacity-[0.85] focus:shadow-none",
    active: "active:opacity-[0.85] active:shadow-none",
    ..Theme::const_default()
  },
  "blue-gray" => Theme {
    background: "bg-gradient-to-tr from-blue-gray-600 to-blue-gray-400",
    color: "text-white",
    shadow: "shadow-md shadow-blue-gray-500/20",
    hover: "hover:shadow-lg hover:shadow-blue-gray-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "gray" => Theme {
    background: "bg-gradient-to-tr from-gray-900 to-gray-800",
    color: "text-white",
    shadow: "shadow-md shadow-gray-900/10",
    hover: "hover:shadow-lg hover:shadow-gray-900/20",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "brown" => Theme {
    background: "bg-gradient-to-tr from-brown-600 to-brown-400",
    color: "text-white",
    shadow: "shadow-md shadow-brown-500/20",
    hover: "hover:shadow-lg hover:shadow-brown-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "deep-orange" => Theme {
    background: "bg-gradient-to-tr from-deep-orange-600 to-deep-orange-400",
    color: "text-white",
    shadow: "shadow-md shadow-deep-orange-500/20",
    hover: "hover:shadow-lg hover:shadow-deep-orange-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "orange" => Theme {
    background: "bg-gradient-to-tr from-orange-600 to-orange-400",
    color: "text-white",
    shadow: "shadow-md shadow-orange-500/20",
    hover: "hover:shadow-lg hover:shadow-orange-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "amber" => Theme {
    background: "bg-gradient-to-tr from-amber-600 to-amber-400",
    color: "text-black",
    shadow: "shadow-md shadow-amber-500/20",
    hover: "hover:shadow-lg hover:shadow-amber-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "yellow" => Theme {
    background: "bg-gradient-to-tr from-yellow-600 to-yellow-400",
    color: "text-black",
    shadow: "shadow-md shadow-yellow-500/20",
    hover: "hover:shadow-lg hover:shadow-yellow-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "lime" => Theme {
    background: "bg-gradient-to-tr from-lime-600 to-lime-400",
    color: "text-black",
    shadow: "shadow-md shadow-lime-500/20",
    hover: "hover:shadow-lg hover:shadow-lime-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "light-green" => Theme {
    background: "bg-gradient-to-tr from-light-green-600 to-light-green-400",
    color: "text-white",
    shadow: "shadow-md shadow-light-green-500/20",
    hover: "hover:shadow-lg hover:shadow-light-green-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "green" => Theme {
    background: "bg-gradient-to-tr from-green-600 to-green-400",
    color: "text-white",
    shadow: "shadow-md shadow-green-500/20",
    hover: "hover:shadow-lg hover:shadow-green-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "teal" => Theme {
    background: "bg-gradient-to-tr from-teal-600 to-teal-400",
    color: "text-white",
    shadow: "shadow-md shadow-teal-500/20",
    hover: "hover:shadow-lg hover:shadow-teal-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "cyan" => Theme {
    background: "bg-gradient-to-tr from-cyan-600 to-cyan-400",
    color: "text-white",
    shadow: "shadow-md shadow-cyan-500/20",
    hover: "hover:shadow-lg hover:shadow-cyan-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "light-blue" => Theme {
    background: "bg-gradient-to-tr from-light-blue-600 to-light-blue-400",
    color: "text-white",
    shadow: "shadow-md shadow-light-blue-500/20",
    hover: "hover:shadow-lg hover:shadow-light-blue-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "blue" => Theme {
    background: "bg-gradient-to-tr from-blue-600 to-blue-400",
    color: "text-white",
    shadow: "shadow-md shadow-blue-500/20",
    hover: "hover:shadow-lg hover:shadow-blue-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "indigo" => Theme {
    background: "bg-gradient-to-tr from-indigo-600 to-indigo-400",
    color: "text-white",
    shadow: "shadow-md shadow-indigo-500/20",
    hover: "hover:shadow-lg hover:shadow-indigo-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "deep-purple" => Theme {
    background: "bg-gradient-to-tr from-deep-purple-600 to-deep-purple-400",
    color: "text-white",
    shadow: "shadow-md shadow-deep-purple-500/20",
    hover: "hover:shadow-lg hover:shadow-deep-purple-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "purple" => Theme {
    background: "bg-gradient-to-tr from-purple-600 to-purple-400",
    color: "text-white",
    shadow: "shadow-md shadow-purple-500/20",
    hover: "hover:shadow-lg hover:shadow-purple-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "pink" => Theme {
    background: "bg-gradient-to-tr from-pink-600 to-pink-400",
    color: "text-white",
    shadow: "shadow-md shadow-pink-500/20",
    hover: "hover:shadow-lg hover:shadow-pink-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
  "red" => Theme {
    background: "bg-gradient-to-tr from-red-600 to-red-400",
    color: "text-white",
    shadow: "shadow-md shadow-red-500/20",
    hover: "hover:shadow-lg hover:shadow-red-500/40",
    active: "active:opacity-[0.85]",
    ..Theme::const_default()
  },
};