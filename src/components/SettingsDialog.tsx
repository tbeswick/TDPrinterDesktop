import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { PrinterSettings } from "../types/printersettings";
import "./SettingsDialog.css";

interface SettingsDialogProps {
  onClose: () => void;
}

export default function SettingsDialog({
  onClose,
}: SettingsDialogProps) {

  const [apiUrl, setApiUrl] = useState("");
  const [password, setPassword] = useState("");

  const [hasPassword, setHasPassword] = useState(false);

  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  const [error, setError] = useState("");

  // ----------------------------------------
  // Load existing settings
  // ----------------------------------------

  useEffect(() => {

    async function loadSettings() {

      try {

        const settings =
          await invoke<PrinterSettings>(
            "get_printer_settings"
          );

        setApiUrl(settings.api_url);
        setHasPassword(settings.has_password);

      } catch (err) {

        console.error(
          "Failed to load printer settings:",
          err
        );

        setError(
          "Unable to load printer settings."
        );

      } finally {

        setLoading(false);

      }
    }

    loadSettings();

  }, []);


  // ----------------------------------------
  // Save settings
  // ----------------------------------------

  async function handleSave() {

    setSaving(true);
    setError("");

    try {

      await invoke(
        "save_printer_settings",
        {
          apiUrl: apiUrl.trim(),
          password,
        }
      );

      // Don't retain the password in React state.
      setPassword("");

      onClose();

    } catch (err) {

      console.error(
        "Failed to save printer settings:",
        err
      );

      setError(
        "Unable to save printer settings."
      );

    } finally {

      setSaving(false);

    }
  }


  // ----------------------------------------
  // Delete stored password
  // ----------------------------------------

  async function handleRemovePassword() {

    try {

      await invoke(
        "delete_printer_password"
      );

      setHasPassword(false);
      setPassword("");

    } catch (err) {

      console.error(
        "Failed to remove password:",
        err
      );

      setError(
        "Unable to remove stored password."
      );
    }
  }


  if (loading) {

    return (
      <div className="settings-overlay">

        <div className="settings-dialog">

          <p>Loading settings...</p>

        </div>

      </div>
    );
  }


  return (
    <div className="settings-overlay">

      <div className="settings-dialog">

        <button
          className="settings-close-btn"
          onClick={onClose}
          aria-label="Close"
        >
          ×
        </button>


        <h2>Printer Settings</h2>


        <div className="settings-field">

          <label htmlFor="api-url">
            Printer API URL
          </label>

          <input
            id="api-url"
            type="text"
            value={apiUrl}
            onChange={(e) =>
              setApiUrl(e.target.value)
            }
            placeholder="http://192.168.1.100:8000"
          />

        </div>


        <div className="settings-field">

          <label htmlFor="api-password">
            API Password
          </label>

          <input
            id="api-password"
            type="password"
            value={password}
            onChange={(e) =>
              setPassword(e.target.value)
            }
            placeholder={
              hasPassword
                ? "Enter new password to replace existing"
                : "Enter API password"
            }
          />

        </div>


        {hasPassword && (

          <div className="password-status">

            <span>
              Password stored securely in Windows.
            </span>

            <button
              type="button"
              onClick={handleRemovePassword}
            >
              Remove
            </button>

          </div>

        )}


        {error && (

          <div className="settings-error">
            {error}
          </div>

        )}


        <div className="settings-buttons">

          <button
            type="button"
            onClick={onClose}
            disabled={saving}
          >
            Cancel
          </button>

          <button
            type="button"
            onClick={handleSave}
            disabled={saving || !apiUrl.trim()}
          >
            {saving ? "Saving..." : "Save"}
          </button>

        </div>

      </div>

    </div>
  );
}