import React from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from "recharts";

import { TemperatureReading } from "../types/temperature";

interface TemperatureChartProps {
  readings: TemperatureReading[];
}

const TemperatureChart: React.FC<TemperatureChartProps> = ({
  readings,
}) => {
  return (
    <div
      style={{
        width: "100%",
        height: "400px",
        padding: "20px",
        boxSizing: "border-box",
      }}
    >
      <ResponsiveContainer width="100%" height="100%">
        <LineChart
          data={readings}
          margin={{
            top: 10,
            right: 30,
            left: 10,
            bottom: 10,
          }}
        >
          <CartesianGrid strokeDasharray="3 3" />

          <XAxis
            dataKey="timestamp"
            tickFormatter={(value) =>
              new Date(value).toLocaleTimeString([], {
                minute: "2-digit",
                second: "2-digit",
              })
            }
          />

          <YAxis
            unit="°C"
            domain={[0, 300]}
          />

          {/* <Tooltip
            labelFormatter={(value) =>
              new Date(value).toLocaleTimeString()
            }
            formatter={(value, name) => [
              `${Number(value).toFixed(1)} °C`,
              name,
            ]}
          /> */}

          <Legend />

          <Line
            type="monotone"
            dataKey="bedTarget"
            name="Bed Target"
            stroke="#ff9800"
            dot={false}
            strokeWidth={2}
          />

          <Line
            type="monotone"
            dataKey="bedTemp"
            name="Bed Temperature"
            stroke="#f44336"
            dot={false}
            strokeWidth={2}
          />

          <Line
            type="monotone"
            dataKey="nozzleTarget"
            name="Nozzle Target"
            stroke="#2196f3"
            dot={false}
            strokeWidth={2}
          />

          <Line
            type="monotone"
            dataKey="nozzleTemp"
            name="Nozzle Temperature"
            stroke="#4caf50"
            dot={false}
            strokeWidth={2}
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
};

export default TemperatureChart;