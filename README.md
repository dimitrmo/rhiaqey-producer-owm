# OWM producer for the rhiaqey platform

## Endpoint

https://openweathermap.org/api/one-call-3

## Settings

```json
{
  "Interval": 15000,
  "Timeout": 5000,
  "APIKey": "some-api-key",
  "Latitude": 36.141563,
  "Longitude": -5.346491,
  "Exclude": [
    "minutely",
    "hourly",
    "daily",
    "alerts"
  ]
}
```

## Sample response

```json
{
  "lat": 36.1416,
  "lon": -5.3465,
  "timezone": "Europe/Gibraltar",
  "timezone_offset": 7200,
  "current": {
    "dt": 1712663094,
    "sunrise": 1712642140,
    "sunset": 1712688577,
    "temp": 291.57,
    "feels_like": 290.64,
    "pressure": 1025,
    "humidity": 45,
    "dew_point": 279.43,
    "uvi": 6.97,
    "clouds": 0,
    "visibility": 10000,
    "wind_speed": 1.03,
    "wind_deg": 90,
    "weather": [
      {
        "id": 800,
        "main": "Clear",
        "description": "clear sky",
        "icon": "01d"
      }
    ]
  },
  "minutely": [
    {
      "dt": 1712663100,
      "precipitation": 0
    },
    {
      "dt": 1712663160,
      "precipitation": 0
    },
    {
      "dt": 1712663220,
      "precipitation": 0
    },
    {
      "dt": 1712663280,
      "precipitation": 0
    },
    {
      "dt": 1712663340,
      "precipitation": 0
    },
    {
      "dt": 1712663400,
      "precipitation": 0
    },
    {
      "dt": 1712663460,
      "precipitation": 0
    },
    {
      "dt": 1712663520,
      "precipitation": 0
    },
    {
      "dt": 1712663580,
      "precipitation": 0
    },
    {
      "dt": 1712663640,
      "precipitation": 0
    },
    {
      "dt": 1712663700,
      "precipitation": 0
    },
    {
      "dt": 1712663760,
      "precipitation": 0
    },
    {
      "dt": 1712663820,
      "precipitation": 0
    },
    {
      "dt": 1712663880,
      "precipitation": 0
    },
    {
      "dt": 1712663940,
      "precipitation": 0
    },
    {
      "dt": 1712664000,
      "precipitation": 0
    },
    {
      "dt": 1712664060,
      "precipitation": 0
    },
    {
      "dt": 1712664120,
      "precipitation": 0
    },
    {
      "dt": 1712664180,
      "precipitation": 0
    },
    {
      "dt": 1712664240,
      "precipitation": 0
    },
    {
      "dt": 1712664300,
      "precipitation": 0
    },
    {
      "dt": 1712664360,
      "precipitation": 0
    },
    {
      "dt": 1712664420,
      "precipitation": 0
    },
    {
      "dt": 1712664480,
      "precipitation": 0
    },
    {
      "dt": 1712664540,
      "precipitation": 0
    },
    {
      "dt": 1712664600,
      "precipitation": 0
    },
    {
      "dt": 1712664660,
      "precipitation": 0
    },
    {
      "dt": 1712664720,
      "precipitation": 0
    },
    {
      "dt": 1712664780,
      "precipitation": 0
    },
    {
      "dt": 1712664840,
      "precipitation": 0
    },
    {
      "dt": 1712664900,
      "precipitation": 0
    },
    {
      "dt": 1712664960,
      "precipitation": 0
    },
    {
      "dt": 1712665020,
      "precipitation": 0
    },
    {
      "dt": 1712665080,
      "precipitation": 0
    },
    {
      "dt": 1712665140,
      "precipitation": 0
    },
    {
      "dt": 1712665200,
      "precipitation": 0
    },
    {
      "dt": 1712665260,
      "precipitation": 0
    },
    {
      "dt": 1712665320,
      "precipitation": 0
    },
    {
      "dt": 1712665380,
      "precipitation": 0
    },
    {
      "dt": 1712665440,
      "precipitation": 0
    },
    {
      "dt": 1712665500,
      "precipitation": 0
    },
    {
      "dt": 1712665560,
      "precipitation": 0
    },
    {
      "dt": 1712665620,
      "precipitation": 0
    },
    {
      "dt": 1712665680,
      "precipitation": 0
    },
    {
      "dt": 1712665740,
      "precipitation": 0
    },
    {
      "dt": 1712665800,
      "precipitation": 0
    },
    {
      "dt": 1712665860,
      "precipitation": 0
    },
    {
      "dt": 1712665920,
      "precipitation": 0
    },
    {
      "dt": 1712665980,
      "precipitation": 0
    },
    {
      "dt": 1712666040,
      "precipitation": 0
    },
    {
      "dt": 1712666100,
      "precipitation": 0
    },
    {
      "dt": 1712666160,
      "precipitation": 0
    },
    {
      "dt": 1712666220,
      "precipitation": 0
    },
    {
      "dt": 1712666280,
      "precipitation": 0
    },
    {
      "dt": 1712666340,
      "precipitation": 0
    },
    {
      "dt": 1712666400,
      "precipitation": 0
    },
    {
      "dt": 1712666460,
      "precipitation": 0
    },
    {
      "dt": 1712666520,
      "precipitation": 0
    },
    {
      "dt": 1712666580,
      "precipitation": 0
    },
    {
      "dt": 1712666640,
      "precipitation": 0
    }
  ],
  "hourly": [
    {
      "dt": 1712660400,
      "temp": 290.93,
      "feels_like": 289.96,
      "pressure": 1025,
      "humidity": 46,
      "dew_point": 279.17,
      "uvi": 6.01,
      "clouds": 0,
      "visibility": 10000,
      "wind_speed": 2.45,
      "wind_deg": 82,
      "wind_gust": 4.1,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712664000,
      "temp": 291.57,
      "feels_like": 290.64,
      "pressure": 1025,
      "humidity": 45,
      "dew_point": 279.43,
      "uvi": 6.97,
      "clouds": 0,
      "visibility": 10000,
      "wind_speed": 3.59,
      "wind_deg": 89,
      "wind_gust": 5.98,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712667600,
      "temp": 291.06,
      "feels_like": 290.11,
      "pressure": 1025,
      "humidity": 46,
      "dew_point": 279.29,
      "uvi": 6.82,
      "clouds": 0,
      "visibility": 10000,
      "wind_speed": 4.95,
      "wind_deg": 84,
      "wind_gust": 9.2,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712671200,
      "temp": 290.7,
      "feels_like": 289.89,
      "pressure": 1025,
      "humidity": 53,
      "dew_point": 281.02,
      "uvi": 5.73,
      "clouds": 12,
      "visibility": 10000,
      "wind_speed": 7.69,
      "wind_deg": 77,
      "wind_gust": 11.98,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712674800,
      "temp": 290.11,
      "feels_like": 289.51,
      "pressure": 1024,
      "humidity": 63,
      "dew_point": 283.03,
      "uvi": 3.98,
      "clouds": 13,
      "visibility": 10000,
      "wind_speed": 10.26,
      "wind_deg": 75,
      "wind_gust": 13.48,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712678400,
      "temp": 289.39,
      "feels_like": 288.9,
      "pressure": 1024,
      "humidity": 70,
      "dew_point": 283.92,
      "uvi": 2.2,
      "clouds": 14,
      "visibility": 10000,
      "wind_speed": 10.55,
      "wind_deg": 72,
      "wind_gust": 13.88,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712682000,
      "temp": 288.69,
      "feels_like": 288.31,
      "pressure": 1025,
      "humidity": 77,
      "dew_point": 285.11,
      "uvi": 0.87,
      "clouds": 13,
      "visibility": 10000,
      "wind_speed": 9.92,
      "wind_deg": 70,
      "wind_gust": 13.74,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712685600,
      "temp": 288.49,
      "feels_like": 288.14,
      "pressure": 1025,
      "humidity": 79,
      "dew_point": 285.05,
      "uvi": 0.2,
      "clouds": 11,
      "visibility": 10000,
      "wind_speed": 9.51,
      "wind_deg": 69,
      "wind_gust": 13.33,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712689200,
      "temp": 288.59,
      "feels_like": 288.23,
      "pressure": 1026,
      "humidity": 78,
      "dew_point": 285.02,
      "uvi": 0,
      "clouds": 3,
      "visibility": 10000,
      "wind_speed": 8.89,
      "wind_deg": 70,
      "wind_gust": 13.67,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712692800,
      "temp": 288.6,
      "feels_like": 288.24,
      "pressure": 1027,
      "humidity": 78,
      "dew_point": 285.11,
      "uvi": 0,
      "clouds": 1,
      "visibility": 10000,
      "wind_speed": 8.4,
      "wind_deg": 80,
      "wind_gust": 12.76,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712696400,
      "temp": 288.69,
      "feels_like": 288.34,
      "pressure": 1028,
      "humidity": 78,
      "dew_point": 285.08,
      "uvi": 0,
      "clouds": 2,
      "visibility": 10000,
      "wind_speed": 7.98,
      "wind_deg": 77,
      "wind_gust": 12.35,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712700000,
      "temp": 288.74,
      "feels_like": 288.37,
      "pressure": 1028,
      "humidity": 77,
      "dew_point": 284.97,
      "uvi": 0,
      "clouds": 6,
      "visibility": 10000,
      "wind_speed": 7.72,
      "wind_deg": 75,
      "wind_gust": 12.28,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712703600,
      "temp": 288.71,
      "feels_like": 288.2,
      "pressure": 1027,
      "humidity": 72,
      "dew_point": 284.06,
      "uvi": 0,
      "clouds": 6,
      "visibility": 10000,
      "wind_speed": 7.49,
      "wind_deg": 71,
      "wind_gust": 12.08,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712707200,
      "temp": 288.47,
      "feels_like": 287.86,
      "pressure": 1027,
      "humidity": 69,
      "dew_point": 283.04,
      "uvi": 0,
      "clouds": 8,
      "visibility": 10000,
      "wind_speed": 7.48,
      "wind_deg": 62,
      "wind_gust": 11.65,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712710800,
      "temp": 288.39,
      "feels_like": 287.72,
      "pressure": 1027,
      "humidity": 67,
      "dew_point": 282.48,
      "uvi": 0,
      "clouds": 2,
      "visibility": 10000,
      "wind_speed": 8.24,
      "wind_deg": 65,
      "wind_gust": 13,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712714400,
      "temp": 288.41,
      "feels_like": 287.74,
      "pressure": 1026,
      "humidity": 67,
      "dew_point": 282.44,
      "uvi": 0,
      "clouds": 8,
      "visibility": 10000,
      "wind_speed": 8.71,
      "wind_deg": 65,
      "wind_gust": 13.49,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712718000,
      "temp": 288.61,
      "feels_like": 287.99,
      "pressure": 1026,
      "humidity": 68,
      "dew_point": 282.78,
      "uvi": 0,
      "clouds": 6,
      "visibility": 10000,
      "wind_speed": 9.17,
      "wind_deg": 72,
      "wind_gust": 14.12,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712721600,
      "temp": 288.61,
      "feels_like": 288.01,
      "pressure": 1026,
      "humidity": 69,
      "dew_point": 283.05,
      "uvi": 0,
      "clouds": 6,
      "visibility": 10000,
      "wind_speed": 9.33,
      "wind_deg": 74,
      "wind_gust": 14.26,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712725200,
      "temp": 288.62,
      "feels_like": 288.02,
      "pressure": 1026,
      "humidity": 69,
      "dew_point": 283.23,
      "uvi": 0,
      "clouds": 19,
      "visibility": 10000,
      "wind_speed": 10.35,
      "wind_deg": 72,
      "wind_gust": 14.7,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712728800,
      "temp": 288.61,
      "feels_like": 288.04,
      "pressure": 1026,
      "humidity": 70,
      "dew_point": 283.38,
      "uvi": 0,
      "clouds": 18,
      "visibility": 10000,
      "wind_speed": 10.77,
      "wind_deg": 70,
      "wind_gust": 15.05,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712732400,
      "temp": 288.45,
      "feels_like": 287.94,
      "pressure": 1027,
      "humidity": 73,
      "dew_point": 283.84,
      "uvi": 0.37,
      "clouds": 9,
      "visibility": 10000,
      "wind_speed": 10.2,
      "wind_deg": 70,
      "wind_gust": 14.51,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712736000,
      "temp": 288.51,
      "feels_like": 288.06,
      "pressure": 1027,
      "humidity": 75,
      "dew_point": 284.4,
      "uvi": 1.31,
      "clouds": 13,
      "visibility": 10000,
      "wind_speed": 10.9,
      "wind_deg": 74,
      "wind_gust": 14.49,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712739600,
      "temp": 288.66,
      "feels_like": 288.25,
      "pressure": 1027,
      "humidity": 76,
      "dew_point": 285.05,
      "uvi": 2.98,
      "clouds": 10,
      "visibility": 10000,
      "wind_speed": 10.49,
      "wind_deg": 75,
      "wind_gust": 14.13,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712743200,
      "temp": 288.88,
      "feels_like": 288.41,
      "pressure": 1028,
      "humidity": 73,
      "dew_point": 284.55,
      "uvi": 5.03,
      "clouds": 8,
      "visibility": 10000,
      "wind_speed": 10.72,
      "wind_deg": 76,
      "wind_gust": 13.97,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712746800,
      "temp": 289.04,
      "feels_like": 288.62,
      "pressure": 1027,
      "humidity": 74,
      "dew_point": 284.82,
      "uvi": 6.85,
      "clouds": 7,
      "visibility": 10000,
      "wind_speed": 10.57,
      "wind_deg": 76,
      "wind_gust": 13.64,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712750400,
      "temp": 289.27,
      "feels_like": 288.79,
      "pressure": 1027,
      "humidity": 71,
      "dew_point": 284.34,
      "uvi": 7.61,
      "clouds": 6,
      "visibility": 10000,
      "wind_speed": 10.07,
      "wind_deg": 72,
      "wind_gust": 12.93,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712754000,
      "temp": 289.11,
      "feels_like": 288.54,
      "pressure": 1026,
      "humidity": 68,
      "dew_point": 283.52,
      "uvi": 7.6,
      "clouds": 5,
      "visibility": 10000,
      "wind_speed": 9.59,
      "wind_deg": 76,
      "wind_gust": 12.59,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712757600,
      "temp": 289.14,
      "feels_like": 288.57,
      "pressure": 1026,
      "humidity": 68,
      "dew_point": 283.52,
      "uvi": 5.94,
      "clouds": 2,
      "visibility": 10000,
      "wind_speed": 9.39,
      "wind_deg": 77,
      "wind_gust": 12.46,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712761200,
      "temp": 289.07,
      "feels_like": 288.47,
      "pressure": 1025,
      "humidity": 67,
      "dew_point": 283.21,
      "uvi": 4.42,
      "clouds": 3,
      "visibility": 10000,
      "wind_speed": 8.5,
      "wind_deg": 76,
      "wind_gust": 11.39,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712764800,
      "temp": 289.09,
      "feels_like": 288.49,
      "pressure": 1025,
      "humidity": 67,
      "dew_point": 283.24,
      "uvi": 2.45,
      "clouds": 4,
      "visibility": 10000,
      "wind_speed": 7.85,
      "wind_deg": 76,
      "wind_gust": 10.78,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712768400,
      "temp": 289.06,
      "feels_like": 288.53,
      "pressure": 1025,
      "humidity": 70,
      "dew_point": 283.89,
      "uvi": 0.97,
      "clouds": 10,
      "visibility": 10000,
      "wind_speed": 6.84,
      "wind_deg": 82,
      "wind_gust": 10.27,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712772000,
      "temp": 288.78,
      "feels_like": 288.3,
      "pressure": 1025,
      "humidity": 73,
      "dew_point": 284.33,
      "uvi": 0.22,
      "clouds": 26,
      "visibility": 10000,
      "wind_speed": 5.01,
      "wind_deg": 86,
      "wind_gust": 8,
      "weather": [
        {
          "id": 802,
          "main": "Clouds",
          "description": "scattered clouds",
          "icon": "03d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712775600,
      "temp": 288.35,
      "feels_like": 287.88,
      "pressure": 1025,
      "humidity": 75,
      "dew_point": 284.23,
      "uvi": 0,
      "clouds": 100,
      "visibility": 10000,
      "wind_speed": 4.29,
      "wind_deg": 90,
      "wind_gust": 6,
      "weather": [
        {
          "id": 804,
          "main": "Clouds",
          "description": "overcast clouds",
          "icon": "04n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712779200,
      "temp": 288.32,
      "feels_like": 287.82,
      "pressure": 1025,
      "humidity": 74,
      "dew_point": 283.95,
      "uvi": 0,
      "clouds": 100,
      "visibility": 10000,
      "wind_speed": 3.79,
      "wind_deg": 90,
      "wind_gust": 5.37,
      "weather": [
        {
          "id": 804,
          "main": "Clouds",
          "description": "overcast clouds",
          "icon": "04n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712782800,
      "temp": 288.23,
      "feels_like": 287.73,
      "pressure": 1026,
      "humidity": 74,
      "dew_point": 283.82,
      "uvi": 0,
      "clouds": 100,
      "visibility": 10000,
      "wind_speed": 3.56,
      "wind_deg": 86,
      "wind_gust": 5.21,
      "weather": [
        {
          "id": 804,
          "main": "Clouds",
          "description": "overcast clouds",
          "icon": "04n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712786400,
      "temp": 288.06,
      "feels_like": 287.51,
      "pressure": 1026,
      "humidity": 73,
      "dew_point": 283.55,
      "uvi": 0,
      "clouds": 99,
      "visibility": 10000,
      "wind_speed": 3.54,
      "wind_deg": 71,
      "wind_gust": 5.21,
      "weather": [
        {
          "id": 804,
          "main": "Clouds",
          "description": "overcast clouds",
          "icon": "04n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712790000,
      "temp": 287.93,
      "feels_like": 287.37,
      "pressure": 1025,
      "humidity": 73,
      "dew_point": 283.3,
      "uvi": 0,
      "clouds": 97,
      "visibility": 10000,
      "wind_speed": 4.14,
      "wind_deg": 66,
      "wind_gust": 6.02,
      "weather": [
        {
          "id": 804,
          "main": "Clouds",
          "description": "overcast clouds",
          "icon": "04n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712793600,
      "temp": 287.85,
      "feels_like": 287.26,
      "pressure": 1025,
      "humidity": 72,
      "dew_point": 283.17,
      "uvi": 0,
      "clouds": 82,
      "visibility": 10000,
      "wind_speed": 4.39,
      "wind_deg": 65,
      "wind_gust": 6.31,
      "weather": [
        {
          "id": 803,
          "main": "Clouds",
          "description": "broken clouds",
          "icon": "04n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712797200,
      "temp": 287.91,
      "feels_like": 287.32,
      "pressure": 1024,
      "humidity": 72,
      "dew_point": 283.2,
      "uvi": 0,
      "clouds": 12,
      "visibility": 10000,
      "wind_speed": 4.63,
      "wind_deg": 69,
      "wind_gust": 6.89,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712800800,
      "temp": 287.91,
      "feels_like": 287.32,
      "pressure": 1024,
      "humidity": 72,
      "dew_point": 283.12,
      "uvi": 0,
      "clouds": 8,
      "visibility": 10000,
      "wind_speed": 4.88,
      "wind_deg": 67,
      "wind_gust": 7.48,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712804400,
      "temp": 287.97,
      "feels_like": 287.39,
      "pressure": 1023,
      "humidity": 72,
      "dew_point": 283.07,
      "uvi": 0,
      "clouds": 41,
      "visibility": 10000,
      "wind_speed": 5.22,
      "wind_deg": 74,
      "wind_gust": 8.18,
      "weather": [
        {
          "id": 802,
          "main": "Clouds",
          "description": "scattered clouds",
          "icon": "03n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712808000,
      "temp": 288.05,
      "feels_like": 287.45,
      "pressure": 1024,
      "humidity": 71,
      "dew_point": 283.12,
      "uvi": 0,
      "clouds": 58,
      "visibility": 10000,
      "wind_speed": 5.46,
      "wind_deg": 70,
      "wind_gust": 8.26,
      "weather": [
        {
          "id": 803,
          "main": "Clouds",
          "description": "broken clouds",
          "icon": "04n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712811600,
      "temp": 288.05,
      "feels_like": 287.42,
      "pressure": 1024,
      "humidity": 70,
      "dew_point": 282.9,
      "uvi": 0,
      "clouds": 67,
      "visibility": 10000,
      "wind_speed": 5.39,
      "wind_deg": 66,
      "wind_gust": 8.02,
      "weather": [
        {
          "id": 803,
          "main": "Clouds",
          "description": "broken clouds",
          "icon": "04n"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712815200,
      "temp": 288.09,
      "feels_like": 287.47,
      "pressure": 1024,
      "humidity": 70,
      "dew_point": 282.87,
      "uvi": 0,
      "clouds": 57,
      "visibility": 10000,
      "wind_speed": 5.6,
      "wind_deg": 75,
      "wind_gust": 8.38,
      "weather": [
        {
          "id": 803,
          "main": "Clouds",
          "description": "broken clouds",
          "icon": "04d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712818800,
      "temp": 288.45,
      "feels_like": 287.86,
      "pressure": 1024,
      "humidity": 70,
      "dew_point": 283.18,
      "uvi": 0.41,
      "clouds": 14,
      "visibility": 10000,
      "wind_speed": 5.71,
      "wind_deg": 76,
      "wind_gust": 8.87,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712822400,
      "temp": 289.18,
      "feels_like": 288.64,
      "pressure": 1025,
      "humidity": 69,
      "dew_point": 283.66,
      "uvi": 1.44,
      "clouds": 13,
      "visibility": 10000,
      "wind_speed": 6.61,
      "wind_deg": 82,
      "wind_gust": 11.29,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712826000,
      "temp": 289.42,
      "feels_like": 288.9,
      "pressure": 1025,
      "humidity": 69,
      "dew_point": 283.9,
      "uvi": 3.28,
      "clouds": 12,
      "visibility": 10000,
      "wind_speed": 8.16,
      "wind_deg": 85,
      "wind_gust": 12.62,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02d"
        }
      ],
      "pop": 0
    },
    {
      "dt": 1712829600,
      "temp": 289.49,
      "feels_like": 288.98,
      "pressure": 1026,
      "humidity": 69,
      "dew_point": 284.25,
      "uvi": 5.53,
      "clouds": 12,
      "visibility": 10000,
      "wind_speed": 9.04,
      "wind_deg": 86,
      "wind_gust": 13.39,
      "weather": [
        {
          "id": 801,
          "main": "Clouds",
          "description": "few clouds",
          "icon": "02d"
        }
      ],
      "pop": 0
    }
  ],
  "daily": [
    {
      "dt": 1712664000,
      "sunrise": 1712642140,
      "sunset": 1712688577,
      "moonrise": 1712643240,
      "moonset": 1712692860,
      "moon_phase": 0.03,
      "summary": "Expect a day of partly cloudy with clear spells",
      "temp": {
        "day": 291.57,
        "min": 283.89,
        "max": 291.57,
        "night": 288.69,
        "eve": 288.49,
        "morn": 283.89
      },
      "feels_like": {
        "day": 290.64,
        "night": 288.34,
        "eve": 288.14,
        "morn": 283.08
      },
      "pressure": 1025,
      "humidity": 45,
      "dew_point": 279.43,
      "wind_speed": 10.55,
      "wind_deg": 72,
      "wind_gust": 13.88,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "clouds": 0,
      "pop": 0,
      "uvi": 6.97
    },
    {
      "dt": 1712750400,
      "sunrise": 1712728457,
      "sunset": 1712775028,
      "moonrise": 1712731620,
      "moonset": 1712783820,
      "moon_phase": 0.06,
      "summary": "Expect a day of partly cloudy with clear spells",
      "temp": {
        "day": 289.27,
        "min": 288.23,
        "max": 289.27,
        "night": 288.23,
        "eve": 288.78,
        "morn": 288.61
      },
      "feels_like": {
        "day": 288.79,
        "night": 287.73,
        "eve": 288.3,
        "morn": 288.04
      },
      "pressure": 1027,
      "humidity": 71,
      "dew_point": 284.34,
      "wind_speed": 10.9,
      "wind_deg": 74,
      "wind_gust": 15.05,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "clouds": 6,
      "pop": 0,
      "uvi": 7.61
    },
    {
      "dt": 1712836800,
      "sunrise": 1712814775,
      "sunset": 1712861479,
      "moonrise": 1712820300,
      "moonset": 0,
      "moon_phase": 0.1,
      "summary": "Expect a day of partly cloudy with clear spells",
      "temp": {
        "day": 289.47,
        "min": 287.85,
        "max": 289.58,
        "night": 289.05,
        "eve": 288.75,
        "morn": 288.09
      },
      "feels_like": {
        "day": 289.14,
        "night": 288.71,
        "eve": 288.51,
        "morn": 287.47
      },
      "pressure": 1026,
      "humidity": 76,
      "dew_point": 285.58,
      "wind_speed": 10.6,
      "wind_deg": 79,
      "wind_gust": 15.72,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "clouds": 9,
      "pop": 0,
      "uvi": 8.57
    },
    {
      "dt": 1712923200,
      "sunrise": 1712901093,
      "sunset": 1712947929,
      "moonrise": 1712909280,
      "moonset": 1712874600,
      "moon_phase": 0.14,
      "summary": "Expect a day of partly cloudy with clear spells",
      "temp": {
        "day": 289.04,
        "min": 288.51,
        "max": 289.1,
        "night": 288.66,
        "eve": 288.56,
        "morn": 288.66
      },
      "feels_like": {
        "day": 288.8,
        "night": 288.38,
        "eve": 288.3,
        "morn": 288.36
      },
      "pressure": 1030,
      "humidity": 81,
      "dew_point": 286.14,
      "wind_speed": 12.13,
      "wind_deg": 75,
      "wind_gust": 16.98,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "clouds": 3,
      "pop": 0,
      "uvi": 9
    },
    {
      "dt": 1713009600,
      "sunrise": 1712987412,
      "sunset": 1713034380,
      "moonrise": 1712998740,
      "moonset": 1712965140,
      "moon_phase": 0.17,
      "summary": "There will be clear sky today",
      "temp": {
        "day": 288.93,
        "min": 288.35,
        "max": 289.01,
        "night": 288.94,
        "eve": 288.61,
        "morn": 288.35
      },
      "feels_like": {
        "day": 288.78,
        "night": 288.74,
        "eve": 288.46,
        "morn": 288.04
      },
      "pressure": 1028,
      "humidity": 85,
      "dew_point": 286.83,
      "wind_speed": 11.4,
      "wind_deg": 76,
      "wind_gust": 15.82,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "clouds": 8,
      "pop": 0,
      "uvi": 9.02
    },
    {
      "dt": 1713096000,
      "sunrise": 1713073731,
      "sunset": 1713120831,
      "moonrise": 1713088620,
      "moonset": 1713055200,
      "moon_phase": 0.21,
      "summary": "You can expect clear sky in the morning, with partly cloudy in the afternoon",
      "temp": {
        "day": 289.61,
        "min": 288.51,
        "max": 289.68,
        "night": 288.51,
        "eve": 289.19,
        "morn": 288.87
      },
      "feels_like": {
        "day": 289.3,
        "night": 288.35,
        "eve": 288.91,
        "morn": 288.56
      },
      "pressure": 1023,
      "humidity": 76,
      "dew_point": 285.8,
      "wind_speed": 8.93,
      "wind_deg": 79,
      "wind_gust": 13.96,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "clouds": 6,
      "pop": 0,
      "uvi": 10
    },
    {
      "dt": 1713182400,
      "sunrise": 1713160051,
      "sunset": 1713207282,
      "moonrise": 1713178680,
      "moonset": 1713144780,
      "moon_phase": 0.25,
      "summary": "You can expect partly cloudy in the morning, with clearing in the afternoon",
      "temp": {
        "day": 292.32,
        "min": 287,
        "max": 293.47,
        "night": 289.38,
        "eve": 291.66,
        "morn": 287
      },
      "feels_like": {
        "day": 291.83,
        "night": 289.17,
        "eve": 291.45,
        "morn": 286.32
      },
      "pressure": 1016,
      "humidity": 59,
      "dew_point": 284.14,
      "wind_speed": 7.37,
      "wind_deg": 249,
      "wind_gust": 10.32,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "clouds": 7,
      "pop": 0,
      "uvi": 10
    },
    {
      "dt": 1713268800,
      "sunrise": 1713246372,
      "sunset": 1713293733,
      "moonrise": 1713268800,
      "moonset": 1713233700,
      "moon_phase": 0.27,
      "summary": "There will be clear sky today",
      "temp": {
        "day": 290.2,
        "min": 288.36,
        "max": 290.51,
        "night": 288.99,
        "eve": 289.92,
        "morn": 288.62
      },
      "feels_like": {
        "day": 289.92,
        "night": 288.64,
        "eve": 289.66,
        "morn": 288.44
      },
      "pressure": 1021,
      "humidity": 75,
      "dew_point": 286,
      "wind_speed": 5.22,
      "wind_deg": 267,
      "wind_gust": 6.86,
      "weather": [
        {
          "id": 800,
          "main": "Clear",
          "description": "clear sky",
          "icon": "01d"
        }
      ],
      "clouds": 0,
      "pop": 0,
      "uvi": 10
    }
  ],
  "alerts": [
    {
      "sender_name": "AEMET. Agencia Estatal de Meteorología",
      "event": "Moderate coastalevent warning",
      "start": 1712710800,
      "end": 1712743199,
      "description": "Viento de Levante de 50 a 61 Km/h (fuerza 7) al oeste del estrecho.",
      "tags": [
        "Coastal event"
      ]
    }
  ]
}
```