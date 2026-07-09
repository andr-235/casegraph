import { describe, it, expect } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useCaseInspector } from "../pages/case-workspace/inspector/useCaseInspector";

describe("useCaseInspector", () => {
  it("starts closed with revision 0", () => {
    const { result } = renderHook(() => useCaseInspector());

    expect(result.current.target).toBeNull();
    expect(result.current.revision).toBe(0);
  });

  it("opens with correct target type and id", () => {
    const { result } = renderHook(() => useCaseInspector());

    act(() => {
      result.current.open("object", "obj-123");
    });

    expect(result.current.target).not.toBeNull();
    expect(result.current.target!.type).toBe("object");
    expect(result.current.target!.id).toBe("obj-123");
    expect(result.current.revision).toBe(0);
  });

  it("opens different target types", () => {
    const { result } = renderHook(() => useCaseInspector());

    act(() => {
      result.current.open("material", "mat-456");
    });

    expect(result.current.target!.type).toBe("material");
    expect(result.current.target!.id).toBe("mat-456");
  });

  it("closes and resets revision", () => {
    const { result } = renderHook(() => useCaseInspector());

    act(() => {
      result.current.open("relation", "rel-789");
    });

    expect(result.current.target).not.toBeNull();

    act(() => {
      result.current.close();
    });

    expect(result.current.target).toBeNull();
    expect(result.current.revision).toBe(0);
  });

  it("switches target when open called again", () => {
    const { result } = renderHook(() => useCaseInspector());

    act(() => {
      result.current.open("object", "obj-first");
    });

    expect(result.current.target!.id).toBe("obj-first");

    act(() => {
      result.current.open("event", "evt-second");
    });

    expect(result.current.target!.type).toBe("event");
    expect(result.current.target!.id).toBe("evt-second");
    expect(result.current.revision).toBe(0);
  });

  it("increments revision on invalidate", () => {
    const { result } = renderHook(() => useCaseInspector());

    act(() => {
      result.current.open("object", "obj-1");
    });

    expect(result.current.revision).toBe(0);

    act(() => {
      result.current.invalidate();
    });

    expect(result.current.revision).toBe(1);

    act(() => {
      result.current.invalidate();
      result.current.invalidate();
    });

    expect(result.current.revision).toBe(3);
  });

  it("invalidate preserves open target", () => {
    const { result } = renderHook(() => useCaseInspector());

    act(() => {
      result.current.open("object", "obj-keep");
      result.current.invalidate();
    });

    expect(result.current.target!.id).toBe("obj-keep");
    expect(result.current.revision).toBe(1);
  });

  it("invalidate increments revision even when closed", () => {
    const { result } = renderHook(() => useCaseInspector());

    act(() => {
      result.current.invalidate();
    });

    // target stays null, but revision still increments
    expect(result.current.target).toBeNull();
    expect(result.current.revision).toBe(1);
  });
});
