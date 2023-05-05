// use slint;

// pub struct Theme;

// impl Theme {
// slint::slint!{export struct Spaces {
//   small: length,
//   medium: length,
//   large: length,
// }

// export struct Durations {
//   fast: duration,
//   medium: duration,
//   slow: duration,
// }

// export struct Typo {
//   body: {
//     size: length,
//     weight: float,
//   },
//   heading: {
//     size: length,
//     weight: float,
//   },
//   caption: {
//     size: length,
//     weight: float,
//   },
// }


// export struct Palette {
//     // primary
//     cherise: brush,
//     black: brush,
//     emerald: brush,
//     amber: brush,
//     azure: brush,
//     white: brush,

//     // cherise
//     cherise-50: brush,
//     cherise-100: brush,
//     cherise-200: brush,
//     cherise-300: brush,
//     cherise-400: brush,
//     cherise-500: brush,
//     cherise-600: brush,
//     cherise-700: brush,
//     cherise-800: brush,
//     cherise-900: brush,

//     // emerald
//     emerald-50: brush,
//     emerald-100: brush,
//     emerald-200: brush,
//     emerald-300: brush,
//     emerald-400: brush,
//     emerald-500: brush,
//     emerald-600: brush,
//     emerald-700: brush,
//     emerald-800: brush,
//     emerald-900: brush,

//     // gradients
//     emerald-amber-gradient: brush,
//     blue-violet-azure-gradient: brush,
//     black-azure-gradient: brush,
//     emerald-cherise-gradient: brush,
// }

// export global Theme {
//     in property <Palette> palette: {
//         // primary
//         cherise: #DD3365,
//         black: #000000,
//         emerald: #7BD99B,
//         amber: #FFBE0B,
//         azure: #3A86FF,
//         white: #FFFFFF,
//         dimmer: #0000007b,

//         // cherise
//         cherise-50: #EEE6FF,
//         cherise-100: #D0C3FF,
//         cherise-200: #AF9AFF,
//         cherise-300: #896FFF,
//         cherise-400: #654EFF,
//         cherise-500: #2F2AFF,
//         cherise-600: #DD3365,
//         cherise-700: #001FF7,
//         cherise-800: #0019F2,
//         cherise-900: #000AEF,

//         // emerald
//         emerald-50: #FBFFE6,
//         emerald-100: #F4FDC0,
//         emerald-200: #EBFC93,
//         emerald-300: #E2FA63,
//         emerald-400: #DEFB3A,
//         emerald-500: #D6F800,
//         emerald-600: #7BD99B,
//         emerald-700: #BBCF00,
//         emerald-800: #ACB700,
//         emerald-900: #D9D9D9,

//         // gradients
//         emerald-amber-gradient: linear-gradient(90deg, #7BD99B 0%, #FFBE0B 100%),
//         blue-violet-azure-gradient: linear-gradient(90deg, #3A86FF 0%, #7B66FF 100%),
//         black-azure-gradient: linear-gradient(90deg, #000000 0%, #3A86FF 100%),
//         emerald-cherise-gradient: linear-gradient(90deg, #7BD99B 0%, #DD3365 100%),
//     };
//   in property <Spaces> spaces: {
//     xsmall: 2px,
//     small: 4px,
//     medium: 8px,
//     large: 16px,
//     xlarge: 32px,
//   };

//   in property <Durations> durations: {
//     fast: 100ms,
//     medium: 250ms,
//     slow: 500ms,
//     xslow: 1000ms,
//   };

//   in property <Typo> typo: {
//     body: {
//       size: 14px,
//       weight: 400,
//       letterSpacing: 0.5px,
//       lineHeight: 1.5,
//     },
//     heading: {
//       size: 24px,
//       weight: 600,
//       letterSpacing: -0.5px,
//       lineHeight: 1.2,
//     },
//     caption: {
//       size: 12px,
//       weight: 400,
//       letterSpacing: 0.25px,
//       lineHeight: 1.2,
//     },
//   };
// }}}